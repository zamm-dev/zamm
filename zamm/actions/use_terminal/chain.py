from typing import Dict, List

from langchain.chains.llm import LLMChain
from pydantic import BaseModel

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

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        command = self.llm(self.prompt.format(**inputs), stop=["\n"]).strip()
        output = self.terminal.run_bash_command(command).rstrip()

        return {"command": command, "output": output}
