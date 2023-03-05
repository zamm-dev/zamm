# Starting a new Python project

Say you want to do the following task:

> Start a new Python project named hello-world

You can do so by following these steps:

## Project initialization

The very first command we run is to use `poetry` to create a new project. After project creation, we enter the newly created project directory.

```bash
$ poetry new hello-world
Created package hello_world in hello-world
$ cd hello-world
```

Then set the version to `0.0.0` because we're not even at the first release yet.

```
$ poetry version 0.0.0
Bumping version from 0.1.0 to 0.0.0
```

## Development infrastructure

Follow [these instructions](/zamm/resources/tutorials/setup/tools/default.md) to set up default Python dev tooling.

That's all! **Don't take any more steps** because the task is now done!