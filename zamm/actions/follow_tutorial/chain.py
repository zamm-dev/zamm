from typing import Dict, List

from langchain.chains.llm import LLMChain
from langchain.prompts.base import BasePromptTemplate
from pydantic import BaseModel

from .prompt import FOLLOW_TUTORIAL_PROMPT


class FollowTutorialChain(LLMChain, BaseModel):
    """Asks LLM for which tutorial to follow."""

    prompt: BasePromptTemplate = FOLLOW_TUTORIAL_PROMPT

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables

    @property
    def output_keys(self) -> List[str]:
        return ["tutorial"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        tutorial = self.llm(self.prompt.format(**inputs), stop=["\n"]).strip()
        return {"tutorial": tutorial}
