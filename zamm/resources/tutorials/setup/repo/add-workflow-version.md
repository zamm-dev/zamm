# Protecting a Github repo branch

Say you want to do the following task:

> Add Python 3.8 to the Github workflows

You can do so by following these steps:

The very first command we run is to see what Github workflows we have in this repo.

```bash
$ ls .github/workflows
lint.yml  test.yml
```

For each file here, we edit its latest Python matrix version (`jobs.build.strategy.matrix.python-version`) to be in accordance with the task. Let's start by editing `.github/workflows/lint.yml`, which currently looks like this:

```yaml
name: lint

on:
  push:
    branches: [main]
  pull_request:

env:
  POETRY_VERSION: "1.3.1"

jobs:
  build:
    name: ZAMM Linting
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

We add Python 3.8 to the matrix like so:

```yaml
name: lint

on:
  push:
    branches: [main]
  pull_request:

env:
  POETRY_VERSION: "1.3.1"

jobs:
  build:
    name: ZAMM Linting
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version:
          - "3.8"
          - "3.9"
          - "3.10"
    steps:
      - uses: actions/checkout@v3
      - name: Install poetry
        run: |
...
```

Follow the above steps a second time to edit `.github/workflows/test.yml`.

Finally, we follow the steps at [`branch-protection.md`](/zamm/resources/tutorials/setup/repo/branch-protection.md) to protect the `main` branch. It needs updated protection after we added the new version.

That's all! **Don't take any more steps** because the task is now done!
