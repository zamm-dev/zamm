# Setting up `pytest` to run async tests

Say you want to start running async tests in your Python project. You can do so by following these steps:

First, install the package `pytest-asyncio` as as dev dependency:

```bash
$ poetry add pytest-asyncio --group dev
The following packages are already present in the pyproject.toml and will be skipped:

  • pytest-asyncio

If you want to update it to the latest compatible version, you can use `poetry update package`.
If you prefer to upgrade it to the latest available version, you can use `poetry add package@latest`.

Nothing to add.
```

Then, edit `pyproject.toml` to configure `pytest` to run async tests automatically. Let's say the existing file looks like this:

```
[tool.poetry]
name = "zamm"
version = "0.0.3"
description = "General automation driver"
...

[tool.pytest.ini_options]
filterwarnings = [
    "error",
    'ignore:There is no current event loop:DeprecationWarning',
]

...
```

You'll want to add `asyncio_mode = "auto"` to the `[tool.pytest.ini_options]` section, like so:

```
[tool.poetry]
name = "zamm"
version = "0.0.3"
description = "General automation driver"
...

[tool.pytest.ini_options]
asyncio_mode = "auto"
filterwarnings = [
    "error",
    'ignore:There is no current event loop:DeprecationWarning',
]

...
```

If the `[tool.pytest.ini_options]` section doesn't yet exist, you should create it.

## Confirmation

Finally, let's create a new test file to test out our new async test config. Let's say we create `tests/test_new_async.py`:

```
async def test_async_setup():
    assert 1 == 1
```

Now we run just that one test to make sure our configuration works and the test actually runs.

```bash
$ poetry run pytest tests/test_new_async.py
============================= test session starts ==============================
platform linux -- Python 3.11.1, pytest-7.2.1, pluggy-1.0.0
rootdir: /home/amos/projects/gpt-experiments/zamm, configfile: pyproject.toml
plugins: Faker-17.1.0, anyio-3.6.2, asyncio-0.20.3
asyncio: mode=Mode.AUTO
collected 1 item                                                               

tests/test_new_async.py .                                                [100%]

============================== 1 passed in 0.03s ===============================
```

That's all! **Don't take any more steps** because the task is now done!