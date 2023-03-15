# Fixing long import formatting

Say you have a long import like this:

```python
from langchain.prompts.chat import ChatPromptTemplate, HumanMessagePromptTemplate, SystemMessagePromptTemplate
```

`black` reformats it into this:

```python
from langchain.prompts.chat import (
    ChatPromptTemplate,
    HumanMessagePromptTemplate,
    SystemMessagePromptTemplate,
)
```

But then `isort` reformats *that* into this:

```python
from langchain.prompts.chat import (ChatPromptTemplate,
                                    HumanMessagePromptTemplate,
                                    SystemMessagePromptTemplate)
```

Clearly we need some way for the two tools to play together nicely. As such, if you want to do a task such as:

> Fix long import lines being continually reformatted in Python projects

You can do so by following these steps:

Follow the instructions at [`isort.md`](/zamm/resources/tutorials/setup/tools/isort.md) to only configure `pyproject.toml` for `isort` usage. There is no need to install `isort` because in this scenario, it is already installed.

That's all! **Don't take any more steps** because the task is now done!