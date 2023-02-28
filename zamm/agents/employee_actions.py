from typing import Callable, Dict, List

from langchain.agents.agent import AgentExecutor
from langchain.chains.sequential import SequentialChain
from langchain.llms.base import BaseLLM
from langchain.prompts import PromptTemplate
from pydantic import root_validator

from zamm.actions.base import Action
from zamm.actions.edit_file import EditFile
from zamm.actions.finish import Finish
from zamm.actions.follow_tutorial import FollowTutorial
from zamm.actions.note import MakeNote
from zamm.actions.use_terminal import UseTerminal, ZTerminal
from zamm.chains.general import ActionChain
from zamm.chains.general.choice.base import ChoiceChain
from zamm.chains.general.choice.prompt import ChoicePromptTemplate
from zamm.chains.general.llm import ZLLMChain
from zamm.prompts.chained import ChainedPromptTemplate
from zamm.prompts.prefixed import Prefix


class LaxSequentialChain(SequentialChain):
    @root_validator(pre=True)
    def validate_chains(cls, values: Dict) -> Dict:
        return values

    def _validate_outputs(self, outputs: Dict[str, str]) -> None:
        pass

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        known_values = inputs.copy()
        for i, chain in enumerate(self.chains):
            outputs = chain(known_values, return_only_outputs=True)
            known_values.update(outputs)
        return known_values


def default_action_chain(
    llm: BaseLLM,
    prefix: Prefix,
    terminal: ZTerminal,
    agent_creator: Callable[[], AgentExecutor],
    choice_prompt: str = "You have a few actions available to accomplish this: ",
):
    thought_chain_prompt = ChainedPromptTemplate(
        "",
        prefix,
        PromptTemplate(
            input_variables=[],
            template="""
Now, the next step in the employee training manual is (quoted below as a single line):

> """.lstrip(),
        ),
    )
    thought_chain = ZLLMChain(
        llm=llm,
        output_key="next_step",
        prompt=thought_chain_prompt,
        default_stop=["\n"],
    )

    final_prefix = ChainedPromptTemplate(
        "",
        prefix,
        PromptTemplate(
            input_variables=["next_step"],
            template="""
Now, the next step in the employee training manual is (quoted below as a single line):

> {next_step}

""".lstrip(),
        ),
    )

    actions: List[Action] = [
        MakeNote.default(llm=llm, prefix=final_prefix),
        UseTerminal.default(llm=llm, prefix=final_prefix, terminal=terminal),
        EditFile.default(llm=llm, prefix=final_prefix),
        FollowTutorial.default(
            llm=llm, prefix=final_prefix, agent_creator=agent_creator
        ),
        Finish.default(),
    ]

    action_choice_template = ChoicePromptTemplate(
        prefix=ChainedPromptTemplate("", final_prefix, choice_prompt),
        choices=[action.choice_text for action in actions],
    )
    action_choice = ChoiceChain(
        llm=llm,
        prompt=action_choice_template,
        choice_num_key="action_num",
        choice_key="action",
    )
    action_chain = ActionChain(option_picker=action_choice, actions=actions)
    outputs = action_chain.output_keys
    # todo: clean up chain input/outputs
    outputs.remove("next_step")
    outputs.remove("agent_scratchpad")
    outputs.remove("task")
    outputs.remove("documentation")

    return LaxSequentialChain(
        input_variables=thought_chain.input_keys,
        output_variables=outputs,
        chains=[
            thought_chain,
            action_chain,
        ],
    )
