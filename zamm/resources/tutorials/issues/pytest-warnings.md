# Ignoring a warning in `pytest`

Say you encounter a warning while running `pytest`:

```bash
$ poetry run pytest
============================= test session starts ==============================
platform linux -- Python 3.11.1, pytest-7.2.2, pluggy-1.0.0
rootdir: /home/amos/projects/gpt-experiments/langchain-contrib, configfile: pyproject.toml
plugins: anyio-3.6.2, Faker-17.6.0, asyncio-0.20.3
asyncio: mode=Mode.AUTO
collected 13 items                                                             

tests/tools/terminal/test_ansi_escapes.py ........                       [ 61%]
tests/tools/terminal/test_terminal.py ....                               [ 92%]
tests/tools/terminal/test_with_agent.py .                                [100%]

=============================== warnings summary ===============================
../../../.cache/pypoetry/virtualenvs/langchain-contrib-mTbtaW20-py3.11/lib/python3.11/site-packages/nest_asyncio.py:45
  /home/amos/.cache/pypoetry/virtualenvs/langchain-contrib-mTbtaW20-py3.11/lib/python3.11/site-packages/nest_asyncio.py:45: DeprecationWarning: There is no current event loop
    loop = events.get_event_loop_policy().get_event_loop()

-- Docs: https://docs.pytest.org/en/stable/how-to/capture-warnings.html
======================== 13 passed, 1 warning in 9.42s =========================
```

If you want to remove that warning, you can do so by following these steps:

The very first thing we do is to edit the file `pyproject.toml` to edit or create the section `tool.pytest.ini_options` to ignore the new warning. For example, if it currently looks like:

```
[tool.poetry]
name = "langchain-contrib"
version = "0.0.0"
description = ""
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
readme = "README.md"
packages = [{include = "langchain_contrib"}]

...

[tool.pytest.ini_options]
asyncio_mode = "auto"

...
```

then we would edit it to look like

```
[tool.poetry]
name = "langchain-contrib"
version = "0.0.0"
description = ""
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
readme = "README.md"
packages = [{include = "langchain_contrib"}]

...

[tool.pytest.ini_options]
asyncio_mode = "auto"
filterwarnings = [
    "error",
    'ignore:There is no current event loop:DeprecationWarning',
]

...
```

## Confirmation

Let's run `pytest` again to confirm that the warning no longer pops up:

```bash
$ poetry run pytest
============================= test session starts ==============================
platform linux -- Python 3.11.1, pytest-7.2.2, pluggy-1.0.0
rootdir: /home/amos/projects/gpt-experiments/langchain-contrib, configfile: pyproject.toml
plugins: anyio-3.6.2, Faker-17.6.0, asyncio-0.20.3
asyncio: mode=Mode.AUTO
collected 13 items                                                             

tests/tools/terminal/test_ansi_escapes.py ........                       [ 61%]
tests/tools/terminal/test_terminal.py ....                               [ 92%]
tests/tools/terminal/test_with_agent.py .                                [100%]

============================= 13 passed in 12.04s ==============================
```

That's all! **Don't take any more steps** because the task is now done!