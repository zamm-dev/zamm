# Protecting a Github repo branch

Say you want to do the following task:

> Protect the `main` branch on Github

You can do so by following these steps:

First check if the Github API token is available. Since this file may be read and executed by an LLM, we avoid executing a command that actually prints the token out to screen.

```bash
$ [[ "$GITHUB_TOKEN" =~ ^github_pat_ ]] && echo "Github API token exists"
Github API token exists
```

Now let's see what the current repository is so that we can edit it.

```bash
$ git remote -v
origin	git@github.com:amosjyng/zamm.git (fetch)
origin	git@github.com:amosjyng/zamm.git (push)
```

Note that the owner is `amosjyng` and the repo is `zamm`. We have been asked to protect the `main` branch. Remember this for the final command.

Next, let's see what Github workflows we have in this repo.

```bash
$ ls .github/workflows
lint.yml  test.yml
```

For each file here, we check its name (`jobs.build.name`) and latest Python matrix version (`jobs.build.strategy.matrix.python-version`).

```bash
$ cat .github/workflows/lint.yml
name: lint

on:
  push:
    branches: [main]
  pull_request:

env:
  POETRY_VERSION: "1.3.1"

jobs:
  build:
    name: Lint
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version:
          - "3.9"
          - "3.10"
    steps:
      - uses: actions/checkout@v3
      - name: Install poetry
        run: |
...
```

The `jobs.build.name` here is `Lint`, and the `jobs.build.strategy.matrix.python-version` has `3.10` and `3.9` as entries. Therefore, check names would be `Lint (3.9)` and `Lint (3.10)`. Remember this for the future.

```bash
$ cat .github/workflows/test.yml
name: test

on:
  push:
    branches: [main]
  pull_request:

env:
  POETRY_VERSION: "1.3.1"
  OPENAI_API_KEY: "dummy"

jobs:
  build:
    name: Tests
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version:
          - "3.9"
          - "3.10"
    steps:
      - uses: actions/checkout@v3
      - name: Install poetry
...
```

The `jobs.build.name` here is `Tests`, and the `jobs.build.strategy.matrix.python-version` has `3.9` and `3.10` as entries. Therefore, check names would be `Tests (3.9)` and `Tests (3.10)`. Remember this for the future.

Now we're ready to build the final command. We collect all the data we remember from above:

- Owner is `amosjyng`
- Repo is `zamm`
- We're protecting the `main` branch
- `Lint (3.9)`, `Lint (3.10)`, `Tests (3.9)`, and `Tests (3.10)` are the entries

And build the command as mentioned at https://docs.github.com/en/rest/branches/branch-protection#update-branch-protection:

```bash
$ curl -X PUT -H "Accept: application/vnd.github+json" -H "Authorization: Bearer $GITHUB_TOKEN" -H "X-GitHub-Api-Version: 2022-11-28" https://api.github.com/repos/amosjyng/zamm/branches/main/protection -d '{"required_status_checks":{"strict":true,"checks":[{"context":"Lint (3.9)","app_id":15368},{"context":"Lint (3.10)","app_id":15368},{"context":"Tests (3.9)","app_id":15368},{"context":"Tests (3.10)","app_id":15368}]},"enforce_admins":true,"required_linear_history":true,"required_pull_request_reviews":null,"restrictions":null}'
{
  "url": "https://api.github.com/repos/amosjyng/zamm/branches/main/protection",
  "required_status_checks": {
    "url": "https://api.github.com/repos/amosjyng/zamm/branches/main/protection/required_status_checks",
    "strict": true,
    "contexts": [
      "Tests (3.10)",
      "Lint (3.10)",
      "Lint (3.9)",
      "Tests (3.9)"
    ],
    "contexts_url": "https://api.github.com/repos/amosjyng/zamm/branches/main/protection/required_status_checks/contexts",
    "checks": [
      {
        "context": "Tests (3.10)",
        "app_id": 15368
      },
      {
        "context": "Lint (3.10)",
        "app_id": 15368
      },
      {
        "context": "Lint (3.9)",
        "app_id": 15368
      },
      {
        "context": "Tests (3.9)",
        "app_id": 15368
      }
    ]
  },
  "required_signatures": {
    "url": "https://api.github.com/repos/amosjyng/zamm/branches/main/protection/required_signatures",
    "enabled": false
  },
  "enforce_admins": {
    "url": "https://api.github.com/repos/amosjyng/zamm/branches/main/protection/enforce_admins",
    "enabled": true
  },
  "required_linear_history": {
    "enabled": true
  },
  ...
}
```

That's all! You now know how to finish tasks of this kind.
