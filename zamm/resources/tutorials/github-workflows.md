# Setting up Github workflows

Say you want to do the following task:

> Set up Github workflows to lint and test project code

You can do so by following these steps:

First we make note of the poetry version.

```bash
$ poetry --version
Poetry (version 1.3.1)
```

Note that the poetry version here is 1.3.1, so that's what we'll use when creating our Github workflows. Adjust accordingly if your output differs.

Now, create the lint workflow file `.github/workflows/lint.yml` with the following contents:

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

Next, create the test workflow file `.github/workflows/test.yml` with the following contents:

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
