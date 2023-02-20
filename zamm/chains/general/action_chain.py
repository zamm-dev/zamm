"""Chain that chooses and performs the next action."""
from typing import Any, Dict, List, Union

from langchain.chains.base import Chain
from pydantic import BaseModel

from zamm.actions.base import Action

from .choice.base import ChoiceChain
from .options_chain import OptionsChain


class DummyChain(Chain, BaseModel):
    @property
    def input_keys(self) -> List[str]:
        raise NotImplementedError()

    @property
    def output_keys(self) -> List[str]:
        return []

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        raise NotImplementedError()


class ActionChain(OptionsChain, BaseModel):
    option_picker: ChoiceChain
    actions: List[Action]
    step_output_key: str = "step_output"

    def __init__(self, actions: List[Action], **kwargs):
        options = [action.chain for action in actions]
        super().__init__(actions=actions, options=options, **kwargs)

    @property
    def output_keys(self) -> List[str]:
        """Possible output keys produced by this chain."""
        return super().output_keys + [self.step_output_key]

    @property
    def choice_num_key(self) -> str:
        return self.option_picker.choice_num_key

    def option_index(self, picker_output: Dict[str, Any]) -> int:
        """The index of the option chosen."""
        return int(picker_output[self.choice_num_key]) - 1

    def __call__(
        self,
        inputs: Union[Dict[str, Any], Any],
        return_only_outputs: bool = False,
    ) -> Dict[str, Any]:
        output = super().__call__(inputs, return_only_outputs)
        action = self.actions[self.option_index(output)]
        output[self.step_output_key] = action.output_type.from_chain_output(output)
        return output
