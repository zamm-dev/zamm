from typing import Any, List

from langchain.prompts.prompt import PromptTemplate

from zamm.prompts import PrefixedPromptTemplate
from zamm.utils import f_join


class TerminalPromptTemplate(PrefixedPromptTemplate):
    terminal_prompt: str = """
You proceed to use the terminal:

```bash
$ """.lstrip()

    @property
    def _prompt_type(self) -> str:
        """Return the prompt type key."""
        return "terminal"

    def format(self, **kwargs: Any) -> str:
        prefix_text = self.prefix.format(**kwargs)
        return f_join(
            "",
            [
                prefix_text,
                self.terminal_prompt,
            ],
        )


TERMINAL_USAGE_PREFIX = """
You proceed to use the terminal:

```bash"""

TERMINAL_USAGE_SUFFIX = "```"


class TerminalUsageLogger(PromptTemplate):
    input_variables: List[str] = ["command", "output"]
    template: str = """
$ {command}
{output}""".lstrip()
