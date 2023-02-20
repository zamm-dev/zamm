from typing import Any, Dict, List

from langchain.chains.base import Chain
from langchain.prompts.base import BasePromptTemplate
from langchain.schema import AgentFinish

from zamm.actions.base import Action
from zamm.agents.step import StepOutput
from zamm.prompts.dummy import DummyPromptTemplate


class FinishChain(Chain):
    @property
    def input_keys(self) -> List[str]:
        """Input keys this chain expects."""
        return []

    @property
    def output_keys(self) -> List[str]:
        """Output keys this chain expects."""
        return []

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        return {}


class FinishOutput(StepOutput):
    logger_template: BasePromptTemplate = DummyPromptTemplate()

    def log(self, **kwargs) -> str:
        return self.decision.log

    @property
    def template_args(self) -> Dict[str, str]:
        """Construct the dict used to render this output"""
        return {}

    @classmethod
    def from_chain_output(cls, output: Dict[str, Any]):
        """Construct the step from chain output"""
        return cls(
            decision=AgentFinish(
                return_values={"output": "Finished task."},
                log="That's all! You now know how to finish tasks of this kind.",
            ),
            observation=None,
        )


class Finish(Action):
    @classmethod
    def default(cls):
        return cls(
            name="Declare the task done",
            output_type=FinishOutput,
            chain=FinishChain(),
        )
