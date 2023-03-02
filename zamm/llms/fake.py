"""Fake LLM wrapper for testing purposes."""
from typing import Any, List, Mapping, Optional

from langchain.llms.base import LLM
from pydantic import BaseModel


class FakeLLM(LLM, BaseModel):
    """Fake LLM wrapper for testing purposes."""

    queries: Optional[Mapping] = None
    sequenced_responses: Optional[List[str]] = None
    """List of responses to return in order.
    Useful if the prompt is too complicated to reconstruct for testing."""
    num_calls: int = 0
    """Keeps track of which sequenced response to return."""
    ensure_and_remove_stop: bool = False
    """Set to true to check that stops are being set, and being set properly.
    `queries` must be modified to have the stop present.
    """

    @property
    def _llm_type(self) -> str:
        """Return type of llm."""
        return "fake"

    def _check_stop(self, result: str, stop: Optional[List[str]]) -> str:
        if self.ensure_and_remove_stop:
            assert stop is not None, "Stop has not been set"
            found_stop = False
            for s in stop:
                if result.endswith(s):
                    found_stop = True
                    result = result[: len(result) - len(s)]
                    break
            assert found_stop, f"Output '{result}' does not end in {stop}"

        return result

    def _call(self, prompt: str, stop: Optional[List[str]] = None) -> str:
        """First try to lookup in queries, else return 'foo' or 'bar'."""
        self.num_calls += 1

        if self.queries is not None:
            return self._check_stop(self.queries[prompt], stop)
        if self.sequenced_responses is not None:
            return self._check_stop(self.sequenced_responses[self.num_calls - 1], stop)
        if stop is None:
            return "foo"
        else:
            return "bar"

    @property
    def _identifying_params(self) -> Mapping[str, Any]:
        return {}
