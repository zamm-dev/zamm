import shlex
from typing import Dict, List

from langchain.chains.llm import LLMChain
from pydantic import BaseModel

from zamm.actions.edit_file import EditFileChain
from zamm.prompts.prefixed import PrefixedPromptTemplate

from .terminal import ZTerminal


class TerminalChain(LLMChain, BaseModel):
    """Asks LLM for terminal input, and executes it"""

    terminal: ZTerminal

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables

    @property
    def output_keys(self) -> List[str]:
        return ["command", "output"]

    def _validate_outputs(self, outputs: Dict[str, str]) -> None:
        if "file_path" in outputs:
            # edit file chain was invoked instead, validation should really be handled
            # there
            return
        return super()._validate_outputs(outputs)

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        command = self.llm(self.prompt.format(**inputs), stop=["\n"]).strip()
        parsed_cmd = shlex.split(command)
        if len(parsed_cmd) == 2 and parsed_cmd[0] == "nano":
            assert isinstance(self.prompt, PrefixedPromptTemplate)
            edit_result = EditFileChain.for_file(
                llm=self.llm, prefix=self.prompt.prefix
            )(
                {
                    **inputs,
                    "file_path": parsed_cmd[1],
                }
            )
            return edit_result

        output = self.terminal.run_bash_command(command).rstrip()
        return {"command": command, "output": output}
