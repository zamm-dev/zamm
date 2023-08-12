# How to set up Git pre-commit hooks with `pre-commit`

It is desirable for some common operations, such as formatting and linting, to be done prior to every single commit. This can be done via Git pre-commit hooks. So, if you are asked to do a task such as:

> Set up Git pre-commit hooks for a Makefile-based project

then you can do so by following these steps:

## Poetry install

The very first command we run is to install the `pre-commit` package.

```bash
$ poetry add pre-commit --group dev
Using version ^3.0.4 for pre-commit

Updating dependencies
Resolving dependencies... (0.3s)

Writing lock file

Package operations: 6 installs, 0 updates, 0 removals

  • Installing cfgv (3.3.1)
  • Installing identify (2.5.18)
  • Installing nodeenv (1.7.0)
  • Installing black (23.1.0)
  • Installing mypy (1.0.1)
  • Installing pre-commit (3.0.4)
```

## Configuration

Now, let's see what's in the Makefile, if it exists.

```bash
$ cat Makefile
.PHONY: format lint test tests clean release

all: format lint test

format:
	poetry run autoflake -r -i --remove-all-unused-imports .
	poetry run black .
	poetry run isort .

lint:
	poetry run mypy .
	poetry run flake8 .
	poetry run black . --check
	poetry run isort . --check

test: tests
tests:
	poetry run pytest -v

clean:
# https://stackoverflow.com/a/41386937/257583
	find . -type f -name '*.py[co]' -delete -o -type d -name __pycache__ -delete

release:
	test -z "$$(git status --porcelain)"
	poetry version patch
	git commit -am "Creating version v$$(poetry version -s)"
	git tag -a -m "Creating version v$$(poetry version -s)" "v$$(poetry version -s)"
	git push --follow-tags
	poetry publish --build --username $$PYPI_USERNAME --password $$PYPI_PASSWORD
```

As mentioned earlier, we're looking to automate common tasks such as formatting and linting. Since both `format` and `lint` targets exist, we'll run both of these in the `pre-commit` configuration.

`pre-commit` expects its configuration to be in `.pre-commit-config.yaml`, so let's create that file.

```yaml
repos:
  - repo: local
    hooks:
      - id: format
        name: Format code
        entry: make format
        language: system
        type: [python]
        pass_filenames: false
      - id: lint
        name: Lint code
        entry: make lint
        language: system
        type: [python]
        pass_filenames: false
```

This keeps us always up-to-date with the versions of the tools used in the project. You can alternatively set up separate predefined hooks for each of the tools you're using. This allows you to check the output of each hook independently in the `pre-commit` output, and removes the need to define these dependencies in your `pyproject.toml`.

```yaml
repos:
- repo: https://github.com/pre-commit/pre-commit-hooks
  rev: v4.4.0
  hooks:
  - id: check-merge-conflict
- repo: https://github.com/astral-sh/ruff-pre-commit
  # Ruff version.
  rev: v0.0.282
  hooks:
    - id: ruff
      args: [ --fix, --exit-non-zero-on-fix ]
- repo: https://github.com/psf/black-pre-commit-mirror
  rev: 23.7.0
  hooks:
  - id: black
- repo: https://github.com/pre-commit/mirrors-mypy
  rev: v1.4.1
  hooks:
  - id: mypy
    args: [--disallow-untyped-defs, --ignore-missing-imports]
```

Additionally, note that the format for the `rev` value differs from repo to repo, some prefixed by `v` and others without. You should check the specific repo to see how their Git tags correlate with their releases.

Additionally, note that for `mypy` in particular, types in dependencies won't be checked unless you manually keep the `additional_dependencies` key in sync with library dependencies: [https://github.com/pre-commit/mirrors-mypy](https://github.com/pre-commit/mirrors-mypy)

If you are running a monorepo with multiple subfolders, you may want to make nested `Makefile`s, and then filter by either setting `files` or `types`. For example, this might be the setup for a Rust subfolder:

```yaml
- repo: local
  hooks:
  - id: format-rust
    name: Format Rust
    entry: make rust-format
    language: system
    types: [rust]
    pass_filenames: false
  - id: lint-rust
    name: Lint Rust
    entry: make rust-lint
    language: system
    types: [rust]
    pass_filenames: false
```

Now set up the Git commit hook scripts with `pre-commit`:

```bash
$ poetry run pre-commit install
pre-commit installed at .git/hooks/pre-commit
```

## Confirmation

Let's now verify this works by creating a new commit.

```bash
$ git add .
$ git commit -m "Set up pre-commit scripts using ZAMM"
Format code..............................................................Passed
Type check...............................................................Passed
[main 9dfee66] Set up pre-commit scripts using ZAMM
 3 files changed, 179 insertions(+), 32 deletions(-)
 create mode 100644 .pre-commit-config.yaml
```

Make sure that lines like this appear in the output:

```
Format code..............................................................Passed
Type check...............................................................Passed
```

**In general**, we should make sure that each type of check is not only capable of passing, but also of failing, or else these checks would give us false confidence into the state of the code. Common ways of triggering errors include:

* For formatting, incorrect whitespace changes, such as indentation for languages that are fine with alternate indentation, or (for Python) spacing around variable declarations
* For type checking, incorrect assignments like creating an integer variable and assigning a string to it
* For linting, unused variable declarations

If you are wondering what types a file corresponds to, you can run

```bash
$ identify-cli src-tauri/src/main.rs
["file", "non-executable", "rust", "text"]
```

In addition, we should check that, for each type of file edited:

- the relevant checks run
- the irrelevant checks *don't* run 

That's all! **Don't take any more steps** because the task is now done!
