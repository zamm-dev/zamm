from typing import Any, Dict, Type

from langchain.chains.base import Chain
from langchain.llms.base import BaseLLM
from langchain.tools.base import BaseTool

from zamm.agents.z_step import DummyStepOutput, ZStepOutput
from zamm.chains.dummy import DummyLLMChain


def dummy_func(input: str) -> str:
    raise NotImplementedError()


class Action(BaseTool):
    name: str
    description: str = "Dummy description"
    chain: Chain
    output_type: Type[ZStepOutput]

    def _run(self, tool_input: str) -> str:
        raise NotImplementedError()

    async def _arun(self, tool_input: str) -> str:
        raise NotImplementedError()

    @property
    def choice_text(self) -> str:
        return self.name

    def use(self, inputs: Dict[str, str]) -> Dict[str, Any]:
        return self.chain(inputs)

    def construct_output(self, output: Dict[str, Any]) -> ZStepOutput:
        """Create a structured representation of tool output."""
        return self.output_type.from_chain_output(output)


class DummyAction(Action):
    def __init__(
        self, llm: BaseLLM, output_type: Type[ZStepOutput] = DummyStepOutput, **kwargs
    ):
        super().__init__(
            chain=DummyLLMChain(llm=llm), output_type=output_type, **kwargs
        )
        assert self.chain is not None
