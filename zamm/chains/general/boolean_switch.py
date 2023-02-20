"""Chain that chooses and performs the next action."""
from abc import abstractmethod
from typing import Any, Dict, List, Optional, Union

from langchain.chains.base import Chain

from .options_chain import OptionsChain


class BooleanSwitchChain(OptionsChain):
    boolean_output_key: Optional[str]

    def __init__(self, true_chain: Chain, false_chain: Chain, **kwargs):
        super().__init__(options=[false_chain, true_chain], **kwargs)

    @property
    def output_keys(self) -> List[str]:
        """Possible output keys produced by this chain."""
        keys = super().output_keys
        if self.boolean_output_key is not None:
            keys.append(self.boolean_output_key)
        return keys

    @abstractmethod
    def condition_was_met(self, outputs: Dict[str, Any]) -> bool:
        """Whether or not the condition was met."""

    def option_index(self, picker_output: Dict[str, Any]) -> int:
        """The index of the option chosen."""
        if self.condition_was_met(picker_output):
            return 1
        return 0

    def __call__(
        self,
        inputs: Union[Dict[str, Any], Any],
        return_only_outputs: bool = False,
    ) -> Dict[str, Any]:
        output = super().__call__(inputs, return_only_outputs)
        if self.boolean_output_key is not None:
            output[self.boolean_output_key] = self.condition_was_met(output)
        return output
