# How to set up Git hooks with `pre-commit`

Say you want to set up Git pre-commit hooks for a Python `poetry` project. You can do so by following these steps:

First, install the `pre-commit` package.

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

Note that there is a `format` target and a `lint` target. On pre-commit, we'll:

- run the `format` target
- Run the `mypy` command, which is the only command in the `lint` target that is not covered by `format` (`autoflake` covers lint issues for `flake8`).

`pre-commit` expects its configuration to be in `.pre-commit-config.yaml`, so let's create that file.

```
repos:
  - repo: local
    hooks:
      - id: format
        name: Format code
        entry: make format
        language: system
        always_run: true
        pass_filenames: false
      - id: mypy
        name: Type check
        entry: poetry run mypy .
        language: system
        always_run: true
        pass_filenames: false
```

And set up the Git commit hook scripts with `pre-commit`:

```bash
$ poetry run pre-commit install
pre-commit installed at .git/hooks/pre-commit
```

Let's now verify this works by running pre-commit on all files in the existing project.

```bash
$ poetry run pre-commit run --all-files
Format code..............................................................Passed
Type check...............................................................Passed
```

Note that since everything passes, we can now save our work!

```bash
$ git add .
$ git commit -m "Set up pre-commit scripts using ZAMM"
Format code..............................................................Passed
Type check...............................................................Passed
[master 9dfee66] Set up pre-commit scripts using ZAMM
 3 files changed, 179 insertions(+), 32 deletions(-)
 create mode 100644 .pre-commit-config.yaml
```

That's all! **Don't take any more steps** because the task is now done!
