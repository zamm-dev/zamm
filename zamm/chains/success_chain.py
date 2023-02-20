"""Chain that interprets a prompt and executes bash code to perform bash operations."""
from typing import Dict, List

from langchain.llms.base import BaseLLM
from langchain.prompts import PromptTemplate
from pydantic import BaseModel, Extra

from ..prompts.chained import ChainedPromptTemplate
from .bash_action_prompt import BASH_COMMAND_TEMPLATE
from .general import ChoiceChain, ChoicePromptTemplate

POST_ACTION_PROMPT = """
{command}
{output}
```

You squint at the screen to determine whether:""".lstrip()

POST_ACTION_TEMPLATE = PromptTemplate(
    input_variables=["command", "output"],
    template=POST_ACTION_PROMPT,
)


COMMAND_RESULT_TEMPLATE = ChoicePromptTemplate(
    prefix=ChainedPromptTemplate("", BASH_COMMAND_TEMPLATE, POST_ACTION_TEMPLATE),
    choices=[
        "The command succeeded as intended",
        "The command failed",
    ],
)


BOOLS = ["True", "False"]


class ZOutputSuccessChain(ChoiceChain, BaseModel):
    """Checks whether or not output succeeded"""

    llm: BaseLLM
    """LLM wrapper to use."""
    prompt: ChoicePromptTemplate = COMMAND_RESULT_TEMPLATE

    class Config:
        """Configuration for this pydantic object."""

        extra = Extra.forbid

    @property
    def output_keys(self) -> List[str]:
        """Output keys produced by this chain."""
        return ["visual_success"]

    def _return_dict(self, choice_num: int, choice: str) -> Dict[str, str]:
        return {"visual_success": BOOLS[choice_num - 1]}
