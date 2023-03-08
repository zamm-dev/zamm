# Setting up `black`

Every project should have automated code formatting set up to ensure a consistent code style, and `black` is a sensible default code formatter for Python projects. As such, if you are given a task such as:

> Set up code formatting in this Python project

then you can set up `black` for your project by following these steps:

The very first command we run is to install `black` as a dev dependency.

```bash
$ poetry add black --group dev
Using version ^23.1.0 for black

Updating dependencies
Resolving dependencies... (0.1s)

Writing lock file

No dependencies to install or update
```

## Confirmation

Let's actually run `black` and check that it works.

```bash
$ poetry run black .
All done! ✨ 🍰 ✨
3 files left unchanged.
```

That's all! **Don't take any more steps** because the task is now done!