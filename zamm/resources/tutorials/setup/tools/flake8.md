# Setting up `flake8` and `autoflake`

Every project should have automated code linting set up for additional style consistency. Common defaults for Python projects in this category include:

- `flake8`, a popular linting tool
- `flake8-docstrings`, a `flake8` plugin that also makes sure our public functions are well-documented
- `autoflake`, a popular tool for fixing lint issues

As such, if you are given a task such as:

> Set up code linting in this Python project

then you can do so by following these steps:

## `poetry` install

The very first command we run is to install `flake8`, `flake8-docstrings` and `autoflake` as dev dependencies.

```bash
$ poetry add flake8 flake8-docstrings autoflake --group dev
Using version ^6.0.0 for flake8
Using version ^1.7.0 for flake8-docstrings
Using version ^2.0.1 for autoflake

Updating dependencies
Resolving dependencies... (0.6s)

Writing lock file

Package operations: 1 install, 0 updates, 0 removals

  • Installing flake8-docstrings (1.7.0)
```

## Configuration files

Let's first configure `autoflake` in the file `pyproject.toml`. Suppose it currently looks like this:

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
autoflake = "^2.0.1"
flake8 = "^6.0.0"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

Then the only change we make is to add a section for `autoflake`, which will cause the file to look like this:

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
autoflake = "^2.0.1"
flake8 = "^6.0.0"

[tool.autoflake]
in-place = true
recursive = true
remove-all-unused-imports = true
remove-duplicate-keys = true
remove-unused-variables = true

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

Our next step is to configure `flake8`. Unfortunately, `flake8` does not support `pyproject.toml`. There are other libraries that patch it to support `pyproject.toml`, but since this tutorial is meant to be a good default, let's stick to the defaults.

We create the file `.flake8` with the contents:

```
[flake8]
# Match black line length (default 88) instead of flake8 default of 79
max-line-length = 88
extend-ignore =
    # See https://github.com/PyCQA/pycodestyle/issues/373
    E203,
```

## Public docstrings

Then, we should make sure all the `__init__.py` files have docstrings now. Let's see which ones there are:

```bash
$ ls **/__init__.py
hello_world/__init__.py  tests/__init__.py
```

So we start off with `hello_world/__init__.py`. Say it's empty:

```

```

We edit the file to contain a single line documentation for the module, making sure to put a newline at the end.

```
"""A brand new project."""

```

The file `tests/__init__.py` should initially have empty contents too:

```

```

We replace the file contents, making sure once again to put a newline at the end. (Note that the project folder in this tutorial is named `hello_world`; adjust the documentation below to fit your project instead.)

```
"""All tests for the hello-world project."""

```

## Confirmation

Let's check that we have configured `autoflake` and `flake8` correctly by running them. **DO NOT DO THIS UNLESS YOU'VE DONE EVERYTHING ABOVE.**

```bash
$ poetry run autoflake .
$ poetry run flake8 .
```

If successful, they should ideally produce no output.

That's all! **Don't take any more steps** because the task is now done!