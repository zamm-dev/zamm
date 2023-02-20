from typing import Any, List

from fvalues import F

from zamm.prompts import PrefixedPromptTemplate
from zamm.utils import f_join


class ChoicePrompt(F):
    prefix_text: str
    choices: List[str]

    def __new__(cls, f_str: F, prefix_text: str, choices: List[str]):
        result = super().__new__(cls, f_str, parts=f_str.parts)
        result.prefix_text = prefix_text
        result.choices = choices
        return result


class ChoicePromptTemplate(PrefixedPromptTemplate):
    choices: List[str]
    choice_prompt: str = "You decide it's #"

    @property
    def _prompt_type(self) -> str:
        """Return the prompt type key."""
        return "choice"

    @property
    def choices_text(self) -> str:
        return "\n".join([f"{i+1}. {choice}" for i, choice in enumerate(self.choices)])

    def format(self, **kwargs: Any) -> str:
        prefix_text = self.prefix.format(**kwargs)
        prompt = f_join(
            "\n\n",
            [
                prefix_text,
                self.choices_text,
                self.choice_prompt,
            ],
        )
        return ChoicePrompt(prompt, prefix_text, self.choices)
