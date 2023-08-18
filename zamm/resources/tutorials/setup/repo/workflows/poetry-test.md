# Setting up Github workflows for Poetry

Say you want to do the following task:

> Set up Github workflows to lint and test project code for Python versions 3.9 to 3.11

You can do so by following these steps:

The very first command we run is to make note of the poetry version.

```bash
$ poetry --version
Poetry (version 1.3.1)
```

Note that the poetry version here is 1.3.1, so that's what we'll use when creating our Github workflows. Adjust accordingly if your output differs.

Now, create the lint workflow file `.github/workflows/lint.yml`. Note that the tasks asks for Python versions from 3.9 to 3.11, so the `python-version` list will therefore contain entries for Python versions 3.9, 3.10, and 3.11.

Also note that if you're using pre-commit, there's [a separate way](https://pre-commit.com/#github-actions-example) to configure it to run, so as to avoid having to install all the pre-commit dependencies as part of the project's own dev dependencies.

```
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
          - "3.11"
    steps:
      - uses: actions/checkout@v3
      - name: Install poetry
        run: |
          pipx install poetry==$POETRY_VERSION
      - name: Set up Python ${{ matrix.python-version }}
        uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python-version }}
          cache: poetry
      - name: Install dependencies
        run: |
          poetry install
      - name: Analysing the code with our lint
        run: |
          make lint
```

Next, create the test workflow file `.github/workflows/test.yml`. Note again that the tasks asks for Python versions from 3.9 to 3.11, so the `python-version` list will therefore contain entries for Python versions 3.9, 3.10, and 3.11.

```
name: test

on:
  push:
    branches: [main]
  pull_request:

env:
  POETRY_VERSION: "1.3.1"

jobs:
  build:
    name: Test
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version:
          - "3.9"
          - "3.10"
          - "3.11"
    steps:
      - uses: actions/checkout@v3
      - name: Install poetry
        run: pipx install poetry==$POETRY_VERSION
      - name: Set up Python ${{ matrix.python-version }}
        uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python-version }}
          cache: "poetry"
      - name: Install dependencies
        run: poetry install
      - name: Run unit tests
        run: |
          make tests
```

Let's **make sure to commit everything** by running these final terminal commands:

```bash
$ git add .
$ git commit -m "Set up Github workflows with ZAMM"
[master (root-commit) 7a31990] Set up Github workflows with ZAMM
 2 files changed, 151 insertions(+)
 create mode 100644 .github/workflows/lint.yml
 create mode 100644 .github/workflows/test.yml
```

That's all! **Don't take any more steps** because the task is now done!