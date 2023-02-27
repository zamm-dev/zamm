import os
import warnings
from typing import Callable, List, Optional

from langchain.llms.base import LLM
from simple_term_menu import TerminalMenu
from vcr.cassette import Cassette
from vcr_langchain.patch import GenericPatch, add_patchers

from zamm.chains.general import ChoicePrompt


class Human(LLM):
    prerecorded_responses: Optional[List[str]] = None
    playback_index: int = 0

    @property
    def _llm_type(self) -> str:
        return "Human"

    def _stop_hit(self, input: str, stops: List[str]) -> Optional[str]:
        for stop in stops:
            if stop in input:
                return stop
        return None

    def _call(self, prompt: str, stop: Optional[List[str]] = None) -> str:
        """Run the LLM on the given prompt and input."""
        if self.prerecorded_responses and self.playback_index < len(
            self.prerecorded_responses
        ):
            response = self.prerecorded_responses[self.playback_index]
            self.playback_index += 1
            total_responses = len(self.prerecorded_responses)
            print(
                f"Replaying response #{self.playback_index} of {total_responses}",
                end="\r",
            )
            return response

        os.system("clear")

        if isinstance(prompt, ChoicePrompt):
            print(prompt.prefix_text)
            print()
            result = TerminalMenu(prompt.choices).show()
            if result is None:
                raise RuntimeError()  # most likely because user hit ctrl-C
            return str(result + 1)

        user_input = input(prompt)

        if stop is None or stop == ["\n"]:
            return user_input

        stop_hit = self._stop_hit(user_input, stop)
        while stop_hit is None:
            new_input = input()
            stop_hit = self._stop_hit(new_input, stop)
            user_input += "\n" + new_input

        separated_inputs = [x for x in user_input.split(stop_hit) if x]
        if separated_inputs == []:
            result = ""
        else:
            result = separated_inputs[0]
            if len(separated_inputs) > 1:
                warnings.warn(
                    f"Ignoring rest of input after stop: {separated_inputs[1:]}"
                )

        return result


class HumanPatch(GenericPatch):
    def __init__(self, cassette: Cassette):
        super().__init__(cassette, Human, "_call")

    def get_same_signature_override(self) -> Callable:
        def _call(og_self: Human, prompt: str, stop: Optional[List[str]] = None) -> str:
            """Same signature override patched into Human"""
            return self.generic_override(og_self, prompt=prompt, stop=stop)

        return _call


add_patchers(HumanPatch)
