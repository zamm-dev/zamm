"""Chain that interprets a prompt and executes bash code to perform bash operations."""
from typing import Dict, List

from langchain.chains.base import Chain
from langchain.llms.base import BaseLLM
from langchain.prompts.base import BasePromptTemplate
from pydantic import BaseModel, Extra

from .bash_action_prompt import SUCCESS_CRITERIA_TEMPLATE


class GetSuccessCriteriaChain(Chain, BaseModel):
    """Custom Bash chain for easier experimentation"""

    llm: BaseLLM
    """LLM wrapper to use."""
    prompt: BasePromptTemplate = SUCCESS_CRITERIA_TEMPLATE

    class Config:
        """Configuration for this pydantic object."""

        extra = Extra.forbid

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return ["documentation", "task"]

    @property
    def output_keys(self) -> List[str]:
        """Output keys produced by this chain."""
        return ["checklist"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        output = self.llm(self.prompt.format(**inputs), stop=["\n\n"]).strip()

        return {"checklist": output}
