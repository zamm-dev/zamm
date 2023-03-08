# Setting up `mypy`

Type-checking helps shorten the feedback loop on finding and fixing errors. Even though Python doesn't enforce types by default, we can use the `mypy` tool to help us out as much as practically possible. As such, if you are given a task such as:

> Set up type checking in this Python project

then you can set up `mypy` for your project by following these steps:

## Poetry install

The very first command we run is to install `mypy` as a dev dependency.

```bash
$ poetry add mypy --group dev
Using version ^1.0.1 for mypy

Updating dependencies
Resolving dependencies... (0.2s)

Writing lock file

No dependencies to install or update
```

## Configuration

We will configure `mypy` to not error out on dependencies that don't have types defined, as well as to rigorously enforce types. Let's do so by configuring `pyproject.toml`, which might look like:

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
mypy = "^1.0.1"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

All we do here is to add a section for `mypy`, causing the file to look like:

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
mypy = "^1.0.1"

[tool.mypy]
ignore_missing_imports = "True"
disallow_untyped_defs = "True"

[build-system]
requires = ["poetry-core"]
build-backend = "poetry.core.masonry.api"
```

## Confirmation

Let's check that we have configured `mypy` correctly by running it.

```bash
$ poetry run mypy .
Success: no issues found in 2 source files
```

That's all! **Don't take any more steps** because the task is now done!