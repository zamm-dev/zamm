# Adding a new agent action

Say you want to do the following task:

> Add a new action

You can do so by following these steps:

## Creating a new package for the action

Let's see what project this is by peeking at the first few lines of `pyproject.toml`:

```bash
$ head -n 5 pyproject.toml
[tool.poetry]
name = "zamm"
version = "0.0.3"
description = "General automation driver"
authors = ["Amos Jun-yeung Ng <me@amos.ng>"]
```

The project is called **zamm**. As such, first we'll create a new package `zamm.actions.follow_tutorial` where we put all our new action logic inside. We start off this package with a file to store all action-related prompts: `zamm/actions/follow_tutorial/prompt.py`. Fill it up with content like this:

```
from langchain.prompts.prompt import PromptTemplate

FOLLOW_TUTORIAL_PROMPT = PromptTemplate(
    input_variables=[],
    template="You follow the tutorial located at: ",
)

FOLLOW_TUTORIAL_LOGGER = PromptTemplate(
    input_variables=["tutorial"],
    template="\nYou followed the tutorial at {tutorial}",
)
```

Then we add a file to define the action-related chain logic: `zamm/actions/follow_tutorial/chain.py`. The chain logic in the file might look something like:

```
from typing import Dict, List

from langchain.chains.llm import LLMChain
from langchain.prompts.base import BasePromptTemplate
from pydantic import BaseModel

from .prompt import FOLLOW_TUTORIAL_PROMPT


class FollowTutorialChain(LLMChain, BaseModel):
    """Asks LLM for which tutorial to follow."""

    prompt: BasePromptTemplate = FOLLOW_TUTORIAL_PROMPT

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables

    @property
    def output_keys(self) -> List[str]:
        return ["tutorial"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        tutorial = self.llm(self.prompt.format(**inputs), stop=["\n"]).strip()
        return {"tutorial": tutorial}
```

Finally, we define the action itself and its output in the file `zamm/actions/follow_tutorial/action.py` with something like:

```
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
```

Finally, we make this a proper Python package by giving it an `__init__.py`. We create the new file `zamm/actions/follow_tutorial/__init__.py` and export all the major classes there.

```
from .action import FollowTutorial

__all__ = [
    "FollowTutorial",
    "FollowTutorialOutput",
    "FollowTutorialChain",
]
```

## Using the new package

Go to the file where the agent's set of possible actions are defined. For the ZAMM project, that would be `zamm/agents/employee_actions.py`. That file might look like this at first:

```
from typing import List

from langchain.llms.base import BaseLLM

from zamm.actions.base import Action
from zamm.actions.edit_file import EditFile
from zamm.actions.finish import Finish
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
    choice_prompt: str = "You now contemplate your next step:",
):
    actions: List[Action] = [
        MakeNote.default(llm=llm, prefix=prefix),
        UseTerminal.default(llm=llm, prefix=prefix, terminal=terminal),
        EditFile.default(llm=llm, prefix=prefix),
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
```

Edit it to create an instance of the new action we just defined:

```
from typing import List

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
    choice_prompt: str = "You now contemplate your next step:",
):
    actions: List[Action] = [
        MakeNote.default(llm=llm, prefix=prefix),
        UseTerminal.default(llm=llm, prefix=prefix, terminal=terminal),
        EditFile.default(llm=llm, prefix=prefix),
        FollowTutorial.default(llm=llm, prefix=prefix),
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
```

## Confirmation

Make sure everything checks out by running formatting and linting. For this project, that would be with the `format` and `lint` commands in the Makefile:

```bash
$ make format lint
poetry run autoflake -r -i --remove-all-unused-imports .
poetry run black .
All done! ✨ 🍰 ✨
...
poetry run isort . --check
Skipped 3 files
poetry run flake8 .
```

Tests will likely fail given that we just changed the prompt for every test that uses this agent, so we can avoid running them.

That's all! **Don't take any more steps** because the task is now done!