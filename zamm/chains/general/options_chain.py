"""Chain that chooses and performs the next action."""
from abc import ABC, abstractmethod
from typing import Any, Dict, List

from langchain.chains.base import Chain
from pydantic import BaseModel

from zamm.utils import safe_call


class OptionsChain(Chain, BaseModel, ABC):
    option_picker: Chain
    options: List[Chain]

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.option_picker.input_keys

    @property
    def output_keys(self) -> List[str]:
        """Possible output keys produced by this chain."""
        all_keys = set(self.option_picker.output_keys)
        for option in self.options:
            all_keys.update(option.output_keys)
        return list(all_keys)

    @abstractmethod
    def option_index(self, picker_output: Dict[str, Any]) -> int:
        """The index of the option chosen."""

    def _validate_outputs(self, outputs: Dict[str, str]) -> None:
        """Always validated because different options may produce different outputs."""

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        """Run the logic of this chain and return the output."""
        picker_output = self.option_picker(inputs)
        option_index = self.option_index(picker_output)
        option = self.options[option_index]
        full_inputs = {**inputs, **picker_output}
        option_output = safe_call(option, full_inputs)
        return {**full_inputs, **option_output}
