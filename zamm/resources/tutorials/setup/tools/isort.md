# Setting up `isort`

Part of ensuring consistent code style is enforcing consistent import conventions, and `isort` is a sensible default import manager for Python projects. Not only does it sort your imports, it can also remove unused ones.  As such, if you are given a task such as:

> Set up import sorting in this Python project

then you can set up `isort` for your project by following these steps:

## Poetry install

The very first command we run is to install `isort` as a dev dependency.

```bash
$ poetry add isort --group dev
Using version ^5.12.0 for isort

Updating dependencies
Resolving dependencies... (0.1s)

Writing lock file

No dependencies to install or update
```

`isort` requires some configuration to play nicely with `black`. We'll put this configuration inside `pyproject.toml`, which is where all tooling configuration for the project should live by default.

Let's open up `pyproject.toml`. Let's say it currently looks like this:

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

Then the only change we make is to add a section for `isort`, which will cause the file to look like:

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

That's all! **Don't take any more steps** because the task is now done!