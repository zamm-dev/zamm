# Setting up `make` for Python projects

`make` is a common tool for automating builds by grouping commands together for build tasks. Even if we're not using a compiled language, we can still use `make` to automate frequent tasks such as linting or formatting our code. So, if you're asked to do a task such as:

> Set up a Makefile for the current Python project

then you can do so by following the below steps. Note that this tutorial assumes you've set up your project according to [ZAMM's default Python tooling](/zamm/resources/tutorials/setup/tools/default.md).

The very first thing we do is to create a `Makefile` **in the current directory** (so edit `./Makefile`) that looks like this:

```
.PHONY: format lint test tests clean release

all: format lint test

format:
	poetry run autoflake .
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
	git checkout main
	git pull
	poetry version patch
	git checkout -b "release/v$$(poetry version -s)"
	git commit -am "Releasing version v$$(poetry version -s)"
	git tag -a -m "Releasing version v$$(poetry version -s)" "v$$(poetry version -s)"
	poetry publish --build --username $$PYPI_USERNAME --password $$PYPI_PASSWORD
# git push at the very end to get Github PR link
	git push --set-upstream origin "release/v$$(poetry version -s)"
# --follow-tags seems to suppress Github message output
	git push --follow-tags
```

## Confirmation

As usual, we check that we've configured `make` successfully by running it:

```bash
$ make
poetry run autoflake .
poetry run black .
reformatted /tmp/zamm/scratch/hello-world/hello_world/__init__.py
reformatted /tmp/zamm/scratch/hello-world/tests/__init__.py
reformatted /tmp/zamm/scratch/hello-world/tests/test_pytest_works.py

All done! ✨ 🍰 ✨
3 files reformatted.
poetry run isort .
Skipped 2 files
poetry run mypy . --exclude scratch
Success: no issues found in 3 source files
poetry run flake8 .
poetry run black . --check
All done! ✨ 🍰 ✨
3 files would be left unchanged.
poetry run isort . --check
Skipped 2 files
poetry run pytest -v
============================= test session starts ==============================
platform linux -- Python 3.10.9, pytest-7.2.1, pluggy-1.0.0 -- /home/amos/.cache/pypoetry/virtualenvs/zamm-gmabLGlH-py3.10/bin/python
cachedir: .pytest_cache
rootdir: /tmp/zamm/scratch/hello-world
plugins: anyio-3.6.2, Faker-17.0.0, asyncio-0.20.3
asyncio: mode=strict
collected 1 item                                                               

tests/test_pytest_works.py::test_pytest_works PASSED                     [100%]

============================== 1 passed in 0.05s ===============================
```

That's all! **Don't take any more steps** because the task is now done!