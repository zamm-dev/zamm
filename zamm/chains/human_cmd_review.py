from typing import Any, Dict

from langchain.chains.llm import LLMChain
from langchain.llms.base import BaseLLM
from langchain.prompts.prompt import PromptTemplate

from zamm.chains.general import ChoiceChain, ChoicePromptTemplate, ErrorChain, NoOpChain
from zamm.chains.general.options_chain import OptionsChain

COMMAND_PROMPT_TEMPLATE = PromptTemplate(
    input_variables=["command"],
    template="The LLM would like to run the command `{command}`",
)

EDIT_COMMAND_PROMPT_TEMPLATE = PromptTemplate(
    input_variables=["command"],
    template="The LLM would like to run the command `{command}`\n\nReplace it with: ",
)

COMMAND_REVIEW_TEMPLATE = ChoicePromptTemplate(
    prefix=COMMAND_PROMPT_TEMPLATE,
    choice_prompt="",
    choices=["Proceed", "Edit command", "Stop (or Ctrl-C)"],
)


class HumanCommandReviewChain(OptionsChain):
    option_picker: ChoiceChain

    @property
    def choice_num_key(self) -> str:
        # todo: have this and action_chain inherit from the same base class
        return self.option_picker.choice_num_key

    def option_index(self, picker_output: Dict[str, Any]) -> int:
        """The index of the option chosen."""
        return int(picker_output[self.choice_num_key]) - 1

    def __init__(self, llm: BaseLLM, **kwargs):
        option_picker = ChoiceChain(
            llm=llm,
            prompt=COMMAND_REVIEW_TEMPLATE,
        )
        options = [
            NoOpChain(),
            LLMChain(
                llm=llm, output_key="command", prompt=EDIT_COMMAND_PROMPT_TEMPLATE
            ),
            ErrorChain(error_message="LLM danger detected."),
        ]
        super().__init__(option_picker=option_picker, options=options, **kwargs)
