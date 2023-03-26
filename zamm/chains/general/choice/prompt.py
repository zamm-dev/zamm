from typing import List

from fvalues import F
from langchain_contrib.prompts import ChainedPromptTemplate
from langchain_contrib.prompts import (
    ChoicePromptTemplate as ContribChoicePromptTemplate,
)
from langchain_contrib.prompts import Templatable
from langchain_contrib.prompts.choice import list_of_choices


class ChoicePrompt(F):
    prefix_text: str
    choices: List[str]

    def __new__(cls, f_str: F, prefix_text: str, choices: List[str]):
        result = super().__new__(cls, f_str, parts=f_str.parts)
        result.prefix_text = prefix_text
        result.choices = choices
        return result


def ChoicePromptTemplate(
    prefix: Templatable, choices: List[str]
) -> ContribChoicePromptTemplate:
    return ContribChoicePromptTemplate.from_base_template(
        base_template=ChainedPromptTemplate(
            subprompts=[
                prefix,
                "{choices}",
                "You decide it's #",
            ],
            joiner="\n\n",
        ),
        choices=choices,
        choices_formatter=list_of_choices,
    )
