from typing import List, Optional

from langchain.llms.openai import OpenAI, OpenAIChat
from langchain.schema import LLMResult

from zamm.utils import artificial_stop

CHATGPT_MODEL = "gpt-3.5-turbo"
ARTIFICIAL_STOPPING = True


class ChatWrapper(OpenAIChat):
    def _generate(
        self, prompts: List[str], stop: Optional[List[str]] = None
    ) -> LLMResult:
        if ARTIFICIAL_STOPPING:
            result = super()._generate(prompts)
            if stop is not None:
                for generation_list in result.generations:
                    for generation in generation_list:
                        generation.text = artificial_stop(generation.text, stop)
            return result

        return super()._generate(prompts, stop)


def new_openai(model_name: str, max_tokens: int = -1, **kwargs):
    if model_name.startswith(CHATGPT_MODEL):
        # todo: support max_tokens for ChatGPT
        return ChatWrapper(model_name=model_name, **kwargs)
    return OpenAI(model_name=model_name, max_tokens=max_tokens, **kwargs)
