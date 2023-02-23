Say you want to do the following task:

> Create a new Python project named hello-world

You can do so by following these steps:



You proceed to use the terminal:

```bash
$ poetry new hello-world
Created package hello_world in hello-world
$ cd hello-world
$ poetry version 0.0.0
Bumping version from 0.1.0 to 0.0.0
```

You note that every project should have automated testing set up, and `pytest` is a sensible default testing framework to use for Python. So let's install `pytest` and make sure it's working.

You proceed to use the terminal:

```bash
$ poetry add pytest --group dev
Using version ^7.2.1 for pytest

Updating dependencies
Resolving dependencies... (0.3s)

Writing lock file

No dependencies to install or update
$ poetry run pytest
============================= test session starts ==============================
platform linux -- Python 3.10.9, pytest-7.2.1, pluggy-1.0.0
rootdir: /tmp/zamm/scratch/hello-world
plugins: anyio-3.6.2, Faker-17.0.0, asyncio-0.20.3
asyncio: mode=strict
collected 0 items                                                              

============================ no tests ran in 0.01s =============================
```

You note that this is a success! `pytest` runs successfully. Let's now turn our attention towards Python linting practices that will keep our projects in consistently good shape. `black` is a Python code formatter that ensures a consistent coding style throughout the codebase.

You proceed to use the terminal:

```bash
$ poetry add black --group dev
Using version ^23.1.0 for black

Updating dependencies
Resolving dependencies... (0.1s)

Writing lock file

No dependencies to install or update
```

You note that next, we should use `isort` to keep our imports sorted and minimized (as in, all unused imports removed from files).

You proceed to use the terminal:

```bash
$ poetry add isort --group dev
Using version ^5.12.0 for isort

Updating dependencies
Resolving dependencies... (0.1s)

Writing lock file

No dependencies to install or update
```

You note that `isort` requires some configuration to play nicely with `black`. We'll put this configuration inside `pyproject.toml`, which is where all tooling configuration for the project should live by default.

You decide to edit the file `pyproject.toml`. Its old contents were

```
[tool.poetry]
name = "hello-world"
version = "0.0.0"
description = ""
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
readme = "README.md"
packages = [{include = "hello_world"}]

[tool.poetry.dependencies]
python = "^3.10"


[tool.poetry.group.dev.dependencies]
pytest = "^7.2.1"
black = "^23.1.0"
isort = "^5.12.0"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"

```

You replace the file contents with

```
[tool.poetry]
name = "hello-world"
version = "0.0.0"
description = ""
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
readme = "README.md"
packages = [{include = "hello_world"}]

[tool.poetry.dependencies]
python = "^3.10"


[tool.poetry.group.dev.dependencies]
pytest = "^7.2.1"
black = "^23.1.0"
isort = "^5.12.0"

[tool.isort]
multi_line_output = 3
include_trailing_comma = true
force_grid_wrap = 0
line_length = 88
profile = "black"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

You note that now, let's setup `flake8`, a popular Python linting tool, and `flake8-docstrings`, a `flake8` plugin that also makes sure our public functions are well-documented.

You proceed to use the terminal:

```bash
$ poetry add flake8 flake8-docstrings --group dev
Using version ^6.0.0 for flake8
Using version ^1.7.0 for flake8-docstrings

Updating dependencies
Resolving dependencies... (0.6s)

Writing lock file

Package operations: 1 install, 0 updates, 0 removals

  • Installing flake8-docstrings (1.7.0): Pending...
[1A[0J  • Installing flake8-docstrings (1.7.0): Installing...
[1A[0J  • Installing flake8-docstrings (1.7.0)
```

You note that `flake8` does not support `pyproject.toml`. There are other libraries that patch it to support `pyproject.toml`, but since we're setting up a generic Python project, let's stick to the defaults.

You decide to edit the file `.flake8`. It doesn't yet exist.

You write out to the file the contents

```
[flake8]
# Match black line length (default 88) instead of flake8 default of 79
max-line-length = 88
extend-ignore =
    # See https://github.com/PyCQA/pycodestyle/issues/373
    E203,
```

You note that we should make sure all the `__init__.py` files have docstrings too now

You proceed to use the terminal:

```bash
$ ls **/__init__.py
hello_world/__init__.py  tests/__init__.py
```

You decide to edit the file `hello_world/__init__.py`. Its old contents were

```

```

You replace the file contents with

```
"""A brand new project."""
```

You decide to edit the file `tests/__init__.py`. Its old contents were

```

```

You replace the file contents with

```
"""All tests for the hello-world project."""
```

You note that let's check that we have configured `flake8` correctly by running it.

You proceed to use the terminal:

```bash
$ poetry run flake8 .
./hello_world/__init__.py:1:27: W292 no newline at end of file
./tests/__init__.py:1:45: W292 no newline at end of file
```

You note that `flake8` does not have the ability to automatically fix linting errors, so let's install `autoflake` for that functionality.

You proceed to use the terminal:

```bash
$ poetry add autoflake --group dev
Using version ^2.0.1 for autoflake

Updating dependencies
Resolving dependencies... (0.1s)

Writing lock file

No dependencies to install or update
```

You note that finally, we use `mypy` to enforce typing as much as possible.

You proceed to use the terminal:

```bash
$ poetry add mypy --group dev
Using version ^1.0.1 for mypy

Updating dependencies
Resolving dependencies... (0.2s)

Writing lock file

No dependencies to install or update
```

You decide to edit the file `pyproject.toml`. Its old contents were

```
[tool.poetry]
name = "hello-world"
version = "0.0.0"
description = ""
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
readme = "README.md"
packages = [{include = "hello_world"}]

[tool.poetry.dependencies]
python = "^3.10"


[tool.poetry.group.dev.dependencies]
pytest = "^7.2.1"
black = "^23.1.0"
isort = "^5.12.0"
flake8 = "^6.0.0"
flake8-docstrings = "^1.7.0"
autoflake = "^2.0.1"
mypy = "^1.0.1"

[tool.isort]
multi_line_output = 3
include_trailing_comma = true
force_grid_wrap = 0
line_length = 88
profile = "black"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

You replace the file contents with

```
[tool.poetry]
name = "hello-world"
version = "0.0.0"
description = ""
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
readme = "README.md"
packages = [{include = "hello_world"}]

[tool.poetry.dependencies]
python = "^3.10"


[tool.poetry.group.dev.dependencies]
pytest = "^7.2.1"
black = "^23.1.0"
isort = "^5.12.0"
flake8 = "^6.0.0"
autoflake = "^2.0.1"
mypy = "^1.0.1"

[tool.isort]
multi_line_output = 3
include_trailing_comma = true
force_grid_wrap = 0
line_length = 88
profile = "black"

[tool.mypy]
ignore_missing_imports = "True"
disallow_untyped_defs = "True"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

You note that check that `mypy` is configured correctly.

You proceed to use the terminal:

```bash
$ poetry run mypy .
Success: no issues found in 2 source files(B
```

You note that any good project requires version control. We'll go with the most popular, `git`.

You proceed to use the terminal:

```bash
$ git init
Initialized empty Git repository in /tmp/zamm/scratch/hello-world/.git/
$ wget https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore -O .gitignore
--2023-02-19 13:57:53--  https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore
Resolving raw.githubusercontent.com (raw.githubusercontent.com)... 185.199.108.133, 185.199.109.133, 185.199.110.133, ...
Connecting to raw.githubusercontent.com (raw.githubusercontent.com)|185.199.108.133|:443... connected.
HTTP request sent, awaiting response... 200 OK
Length: 3078 (3.0K) [text/plain]
Saving to: ‘.gitignore’

.gitignore          100%[===================>]   3.01K  --.-KB/s    in 0s      

2023-02-19 13:57:53 (81.4 MB/s) - ‘.gitignore’ saved [3078/3078]
```

You note that let's automate common tasks with `make`, a classic tool for managing your build tasks.

You decide to edit the file `Makefile`. It doesn't yet exist.

You write out to the file the contents

```
.PHONY: format lint test tests clean release

all: format lint test

format:
	poetry run autoflake -r -i --remove-all-unused-imports .
	poetry run black .
	poetry run isort .

lint:
	poetry run mypy . --exclude scratch
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

You note that we should create a test file so that `pytest` detects and runs our tests successfully.

You decide to edit the file `tests/test_pytest_works.py`. It doesn't yet exist.

You write out to the file the contents

```
"""Dummy test file."""


def test_pytest_works() -> None:
    """Make sure pytest can find and execute this test."""
    assert 1 == 1
```

You note that as usual, we check that we've configured `make` successfully.

You proceed to use the terminal:

```bash
$ make
poetry run autoflake -r -i --remove-all-unused-imports .
poetry run black .
reformatted /tmp/zamm/scratch/hello-world/hello_world/__init__.py
reformatted /tmp/zamm/scratch/hello-world/tests/__init__.py
reformatted /tmp/zamm/scratch/hello-world/tests/test_pytest_works.py

All done! ✨ 🍰 ✨
3 files reformatted.
poetry run isort .
Skipped 2 files
poetry run mypy . --exclude scratch
Success: no issues found in 3 source files(B
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

You note that we need to set up licensing for this repo. We can start with the MIT license by default. However, the license includes a copyright message at the top bearing the copyright-holder's name. If you're an LLM, one trick you can do is to get the user's name from their Git config using the terminal:

You proceed to use the terminal:

```bash
$ git config --global user.name
Amos Jun-yeung Ng
```

You note that the name being output here. For me, it is "Amos Jun-yeung Ng" because that is my name. Whatever the output of this command is, put it into the `LICENSE` file when writing the MIT license.

You decide to edit the file `LICENSE`. It doesn't yet exist.

You write out to the file the contents

```
MIT License

Copyright (c) 2023 Amos Jun-yeung Ng

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

You proceed to use the terminal:

```bash
$ git add .
$ git commit -m "Initial commit with ZAMM"
[master (root-commit) 7a31990] Initial commit with ZAMM
 12 files changed, 751 insertions(+)
 create mode 100644 .flake8
 create mode 100644 .github/workflows/lint.yml
 create mode 100644 .github/workflows/test.yml
 create mode 100644 .gitignore
 create mode 100644 LICENSE
 create mode 100644 Makefile
 create mode 100644 README.md
 create mode 100644 hello_world/__init__.py
 create mode 100644 poetry.lock
 create mode 100644 pyproject.toml
 create mode 100644 tests/__init__.py
 create mode 100644 tests/test_pytest_works.py
```

You note that make sure no files are left behind

You proceed to use the terminal:

```bash
$ git status
On branch master
nothing to commit, working tree clean
```
That's all! **Don't take any more steps** because the task is now done!