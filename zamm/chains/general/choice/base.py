import re
from typing import Dict, List

from langchain.chains.llm import LLMChain
from langchain.llms.base import BaseLLM
from langchain_contrib.prompts import ChoicePromptTemplate
from pydantic import BaseModel


class ChoiceChain(LLMChain, BaseModel):
    """Returns LLM selection of a variety of options"""

    prompt: ChoicePromptTemplate
    choice_num_key: str = "choice_num"
    choice_key: str = "choice"

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables

    @property
    def output_keys(self) -> List[str]:
        return [self.choice_num_key, self.choice_key]

    def _return_dict(self, choice_num: int, choice: str) -> Dict[str, str]:
        return {self.choice_num_key: str(choice_num), self.choice_key: choice}

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        prompt = self.prompt.format(**inputs)
        assert isinstance(self.llm, BaseLLM)
        result = self.llm(prompt, stop=[" "])

        regex_search = re.search(r"\d+", result)
        if regex_search is None:  # likely human LLM
            choice = result
            choice_num = self.prompt.choices.index(choice) + 1
        else:
            choice_num = int(regex_search.group())
            if not 1 <= choice_num <= len(self.prompt.choices):
                raise Exception(f"Unknown choice: '{result}'")
            choice = self.prompt.choices[choice_num - 1]

        return self._return_dict(choice_num, choice)
