# Setting up `pytest`

Every project should have automated testing set up, and `pytest` is a sensible default testing framework to use for Python projects. You can set up `pytest` for your project by following these steps:

First, install `pytest` as as dev dependency.

```bash
$ poetry add pytest --group dev
Using version ^7.2.1 for pytest

Updating dependencies
Resolving dependencies... (0.3s)

Writing lock file

No dependencies to install or update
```

Now, we should create a test file so that we can check if `pytest` detects and runs our tests successfully. Create the file `tests/test_pytest_works.py`:

```
"""Dummy test file."""


def test_pytest_works() -> None:
    """Make sure pytest can find and execute this test."""
    assert 1 == 1
```

## Confirmation

Let's actually run `pytest` and check that it finds the file `test_pytest_works` and runs the single test in it.

```bash
$ poetry run pytest -v
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