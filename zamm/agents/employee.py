from typing import Any, Dict, List, Optional

from langchain.agents.agent import AgentExecutor
from langchain.agents.tools import Tool
from langchain.llms.base import BaseLLM
from langchain.schema import AgentAction, AgentFinish

from zamm.actions.base import Action
from zamm.actions.edit_file import EditFile
from zamm.actions.finish import Finish
from zamm.actions.note import MakeNote
from zamm.actions.use_terminal import UseTerminal, ZTerminal
from zamm.chains.bash_action_prompt import (
    EMPLOYEE_TEACHING_INTRO_TEMPLATE,
    FOLLOW_INSTRUCTIONS_TEMPLATE,
)
from zamm.chains.general import ActionChain
from zamm.chains.general.choice.base import ChoiceChain
from zamm.chains.general.choice.prompt import ChoicePromptTemplate
from zamm.prompts.chained import ChainedPromptTemplate
from zamm.prompts.prefixed import Prefix
from zamm.utils import f_join

from .base import CustomAgent
from .memory import AgentMemory, BaseAgentMemory
from .step import StepOutput


class ZammEmployeeBrain(CustomAgent):
    condense_memory: bool = False

    @property
    def _agent_type(self) -> str:
        """Return Identifier of agent type."""
        return "zamm-employee"

    def _get_next_action(self, full_inputs: Dict[str, str]) -> AgentAction:
        raise NotImplementedError()

    def _construct_scratchpad_base(
        self, memory: BaseAgentMemory, condensed: bool
    ) -> str:
        logs = []
        steps = memory.steps()
        for i, output in enumerate(steps):
            previous = steps[i - 1] if i > 0 else None
            next = steps[i + 1] if i < len(steps) - 1 else None
            logs.append(output.log(previous=previous, next=next, condensed=condensed))
        scratch = f_join("\n", logs)
        if scratch == "":
            return scratch
        return "\n" + scratch + "\n"

    def _construct_scratchpad_structured(self, memory: BaseAgentMemory) -> str:
        return self._construct_scratchpad_base(memory, condensed=self.condense_memory)

    def _construct_scratchpad_final(self, memory: BaseAgentMemory) -> str:
        return self._construct_scratchpad_base(memory, condensed=False)


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


class ZammEmployee(AgentExecutor):
    agent: ZammEmployeeBrain
    terminal: ZTerminal
    max_iterations: Optional[int] = 100

    def __init__(
        self,
        llm: BaseLLM,
        condense_memory: bool = False,
        tools: Optional[List[Tool]] = None,
        terminal_safe_mode: bool = True,
        **kwargs,
    ):
        tools = tools if tools else []
        brain = ZammEmployeeBrain.from_llm(llm)
        brain.condense_memory = condense_memory
        super().__init__(
            agent=brain,
            tools=tools,
            terminal=ZTerminal(safe_mode=terminal_safe_mode),
            **kwargs,
        )

    @property
    def input_keys(self) -> List[str]:
        return ["task", "documentation"]

    def _custom_take_next_step(self, memory: BaseAgentMemory) -> StepOutput:
        if "documentation" in memory.inputs:
            prefix = FOLLOW_INSTRUCTIONS_TEMPLATE
        else:
            prefix = EMPLOYEE_TEACHING_INTRO_TEMPLATE

        action_chain = default_action_chain(
            llm=self.agent.llm_chain.llm,
            prefix=prefix,
            terminal=self.terminal,
        )
        results = action_chain(
            {
                **memory.inputs,
                "agent_scratchpad": self.agent._construct_scratchpad_structured(memory),
            }
        )
        return results[action_chain.step_output_key]

    def _return_structured(
        self, output: StepOutput, memory: AgentMemory
    ) -> Dict[str, Any]:
        memory.add_step(output)
        task = memory.inputs["task"]
        logs = self.agent._construct_scratchpad_final(memory)
        tutorial = f"""
Say you want to do the following task:

> {task}

You can do so by following these steps:

{logs}
""".strip()
        return {"output": tutorial}

    def _validate_inputs(self, inputs: Dict[str, str]) -> None:
        """Skip validation because some inputs are optional."""

    def _call(self, inputs: Dict[str, str]) -> Dict[str, Any]:
        """Stripped down version of AgentExecutor._call."""
        # Do any preparation necessary when receiving a new input.
        self.agent.prepare_for_new_call()

        memory = AgentMemory(inputs=inputs)
        iterations = 0
        # We now enter the agent loop (until it returns something).
        while self._should_continue(iterations):
            step_result = self._custom_take_next_step(memory)
            # If the tool chosen is the finishing tool, then we end and return.
            if isinstance(step_result.decision, AgentFinish):
                return self._return_structured(step_result, memory)

            memory.add_step(step_result)
            iterations += 1
        output = self.agent.return_stopped_response(
            self.early_stopping_method, memory.as_intermediate_steps(), **inputs
        )
        return self._return(output, memory.as_intermediate_steps())
