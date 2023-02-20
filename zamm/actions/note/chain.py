from typing import Dict, List

from langchain.chains.llm import LLMChain
from pydantic import BaseModel


class NoteChain(LLMChain, BaseModel):
    """Asks LLM for what it notes"""

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables

    @property
    def output_keys(self) -> List[str]:
        return ["note"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        note = self.llm(self.prompt.format(**inputs), stop=["\n"]).strip()
        return {"note": note}
