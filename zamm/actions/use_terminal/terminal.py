import os
import shlex
import time
from typing import Callable

import pexpect
from pydantic import BaseModel
from vcr.cassette import Cassette
from vcr_langchain.patch import GenericPatch, add_patchers

from zamm.chains.human_cmd_review import HumanCommandReviewChain
from zamm.llms.human import Human
from zamm.utils import remove_ansi_escapes

ALWAYS_EXECUTE = False


class UnknownResult(Exception):
    pass


class ZTerminal(BaseModel):
    """A virtual terminal that supports interactive shell commands."""

    shell: pexpect.spawn
    bash_prompt: str
    refresh_interval: float
    output_size: int
    safe_mode: bool = True

    class Config:
        arbitrary_types_allowed = True

    def __init__(
        self, refresh_interval: float = 0.1, output_size: int = 1000, **kwargs
    ):
        sh = pexpect.spawn("/bin/bash", encoding="utf-8")
        time.sleep(refresh_interval)
        bash_prompt = sh.read_nonblocking(size=output_size)
        super().__init__(
            refresh_interval=refresh_interval,
            output_size=output_size,
            shell=sh,
            bash_prompt=bash_prompt,
            **kwargs,
        )

    @property
    def prompt_length(self) -> int:
        return len(self.bash_prompt)

    def _get_raw_shell_update_uncached(self, cmd: str) -> str:
        self.shell.sendline(cmd)
        results = ""
        try:
            while not results.endswith(self.bash_prompt):
                latest_output = self.shell.read_nonblocking(size=self.output_size)
                results += latest_output
                time.sleep(0.1)
        except pexpect.TIMEOUT as e:
            raise UnknownResult(
                "Terminal output does not have initial prompt of: "
                f"'{self.bash_prompt}':\n\n{results}"
            ) from e

        # todo: more robust way of syncing terminal actions to action chain state
        parsed_cmd = shlex.split(cmd)
        if len(parsed_cmd) == 2 and parsed_cmd[0] == "cd":
            os.chdir(parsed_cmd[1])

        return results

    def _get_raw_shell_update(self, cmd: str) -> str:
        return self._get_raw_shell_update_uncached(cmd)

    def _is_terminal_state_command(self, cmd: str) -> bool:
        return cmd.startswith("cd ")

    def _get_shell_update(self, cmd: str) -> str:
        if ALWAYS_EXECUTE or self._is_terminal_state_command(cmd):
            return self._get_raw_shell_update_uncached(cmd)
        return self._get_raw_shell_update(cmd)

    def run_bash_command(self, cmd: str) -> str:
        if self.safe_mode:
            cmd = HumanCommandReviewChain(llm=Human())(cmd)["command"]
        results = self._get_shell_update(cmd)
        assert results.startswith(cmd), (
            f"'{results}' does not start with '{cmd}'. "
            "Is non-ASCII terminal input involved?"
        )
        unix_results = results.replace("\r\n", "\n")
        # + 1 to remove leading "\n" after command input
        output = unix_results[len(cmd) + 1 : -self.prompt_length]
        without_ansi = remove_ansi_escapes(output)
        return without_ansi


class ZTerminalPatch(GenericPatch):
    def __init__(self, cassette: Cassette):
        super().__init__(cassette, ZTerminal, "_get_raw_shell_update")

    def get_same_signature_override(self) -> Callable:
        def _call(og_self: ZTerminal, cmd: str) -> str:
            """Same signature override patched into Human"""
            return self.generic_override(og_self, cmd=cmd)

        return _call


add_patchers(ZTerminalPatch)
