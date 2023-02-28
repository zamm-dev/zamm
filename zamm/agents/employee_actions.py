from typing import Callable, List

from langchain.agents.agent import AgentExecutor
from langchain.llms.base import BaseLLM

from zamm.actions.base import Action
from zamm.actions.edit_file import EditFile
from zamm.actions.finish import Finish
from zamm.actions.follow_tutorial import FollowTutorial
from zamm.actions.note import MakeNote
from zamm.actions.use_terminal import UseTerminal, ZTerminal
from zamm.chains.general import ActionChain
from zamm.chains.general.choice.base import ChoiceChain
from zamm.chains.general.choice.prompt import ChoicePromptTemplate
from zamm.prompts.chained import ChainedPromptTemplate
from zamm.prompts.prefixed import Prefix


def default_action_chain(
    llm: BaseLLM,
    prefix: Prefix,
    terminal: ZTerminal,
    agent_creator: Callable[[], AgentExecutor],
    choice_prompt: str = "You now contemplate your next step:",
):
    actions: List[Action] = [
        MakeNote.default(llm=llm, prefix=prefix),
        UseTerminal.default(llm=llm, prefix=prefix, terminal=terminal),
        EditFile.default(llm=llm, prefix=prefix),
        FollowTutorial.default(llm=llm, prefix=prefix, agent_creator=agent_creator),
        Finish.default(),
    ]

    action_choice_template = ChoicePromptTemplate(
        prefix=ChainedPromptTemplate("", prefix, choice_prompt),
        choices=[action.choice_text for action in actions],
    )
    action_choice = ChoiceChain(
        llm=llm,
        prompt=action_choice_template,
        choice_num_key="action_num",
        choice_key="action",
    )

    return ActionChain(option_picker=action_choice, actions=actions)
