from typing import Any, Dict

from langchain.llms.base import BaseLLM
from langchain.schema import AgentAction

from zamm.actions.base import Action
from zamm.agents.z_step import ZStepOutput
from zamm.prompts.chained import ChainedPromptTemplate
from zamm.prompts.prefixed import Prefix

from .chain import FollowTutorialChain
from .prompt import FOLLOW_TUTORIAL_LOGGER, FOLLOW_TUTORIAL_PROMPT


class FollowTutorialOutput(ZStepOutput):
    tutorial: str

    @classmethod
    def from_chain_output(cls, output: Dict[str, Any]):
        return cls(
            decision=AgentAction(
                tool=output["action"],
                tool_input="dummy input",
                log="dummy log",
            ),
            observation="dummy observation",
            tutorial=output["tutorial"],
            logger_template=FOLLOW_TUTORIAL_LOGGER,
        )

    @property
    def template_args(self) -> Dict[str, str]:
        """Construct the dict used to render this output"""
        return {"tutorial": self.tutorial}


class FollowTutorial(Action):
    @classmethod
    def default(cls, llm: BaseLLM, prefix: Prefix):
        return cls(
            name="Follow a tutorial",
            output_type=FollowTutorialOutput,
            chain=FollowTutorialChain(
                llm=llm,
                prompt=ChainedPromptTemplate("", prefix, FOLLOW_TUTORIAL_PROMPT),
            ),
        )
