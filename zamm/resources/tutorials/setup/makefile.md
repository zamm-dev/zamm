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

...

tests/test_pytest_works.py::test_pytest_works PASSED                     [100%]

============================== 1 passed in 0.05s ===============================
```

Note that **it's okay if some of the steps fail**. The only goal here is to make sure that we've set up `make` successfully, not whether or not the make commands actually work. This would be an example of an incorrect Makefile setup which necessitates further probing:

```bash
$ make
make: *** No targets specified and no makefile found.  Stop.
```

Otherwise, **DECLARE THE TASK DONE**. That's all! **Don't take any more steps** because the task is now done!