from typing import Any, Dict, List, Optional, Tuple

from langchain.chains.llm import LLMChain
from langchain.schema import PromptValue


class ZLLMChain(LLMChain):
    default_stop: Optional[List[str]] = None

    def prep_prompts(
        self, input_list: List[Dict[str, Any]]
    ) -> Tuple[List[PromptValue], Optional[List[str]]]:
        prompts, stop = super().prep_prompts(input_list)
        if stop is None:
            stop = self.default_stop
        return prompts, stop
