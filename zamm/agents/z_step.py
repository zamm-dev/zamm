from abc import abstractmethod
from typing import Any, Dict

from langchain.prompts.base import BasePromptTemplate

from zamm.prompts.dummy import DummyPromptTemplate

from .step import StepOutput


class ZStepOutput(StepOutput):
    logger_template: BasePromptTemplate

    def log(self, **kwargs) -> str:
        return self._log(kwargs["previous"], kwargs["next"])

    def _log(self, previous: StepOutput | None, next: StepOutput | None) -> str:
        return self.logger_template.format(**self.template_args)

    @classmethod
    @abstractmethod
    def from_chain_output(cls, output: Dict[str, Any]):
        """Construct the step from chain output"""

    @property
    @abstractmethod
    def template_args(self) -> Dict[str, str]:
        """Construct the dict used to render this output"""


class DummyStepOutput(ZStepOutput):
    logger_template: BasePromptTemplate = DummyPromptTemplate()

    @classmethod
    def from_chain_output(cls, output: Dict[str, Any]):
        """Construct the step from chain output"""
        raise NotImplementedError()

    @property
    def template_args(self) -> Dict[str, str]:
        """Construct the dict used to render this output"""
        raise NotImplementedError()
