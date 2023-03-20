from typing import Any, List

from langchain.prompts import PromptTemplate
from langchain.prompts.base import StringPromptTemplate

from zamm.prompts.prefixed import Prefix

from ..utils import f_join, safe_format


class ChainedPromptTemplate(StringPromptTemplate):
    joiner: str = ""
    subprompts: List[StringPromptTemplate]

    def __init__(self, joiner: str, *subprompts: Prefix):
        prompts: List[StringPromptTemplate] = []
        for subprompt in subprompts:
            if isinstance(subprompt, str):
                if subprompt != "":  # ignore empty strings
                    prompts.append(
                        PromptTemplate(input_variables=[], template=subprompt)
                    )
            elif isinstance(subprompt, StringPromptTemplate):
                prompts.append(subprompt)
            else:
                raise ValueError(f"Subprompt {subprompt} has unknown type")
        input_variables = list(
            set([var for subprompt in prompts for var in subprompt.input_variables])
        )
        super().__init__(
            joiner=joiner,
            subprompts=prompts,
            input_variables=input_variables,
        )

    def format(self, **kwargs: Any) -> str:
        """Format the prompt with the inputs."""
        results = [safe_format(prompt, kwargs) for prompt in self.subprompts]
        return f_join(self.joiner, results)

    @property
    def _prompt_type(self) -> str:
        """Return the prompt type key."""
        return "chained"
