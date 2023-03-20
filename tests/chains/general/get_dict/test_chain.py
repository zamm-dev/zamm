"""Test that GetMultipleOutputsChain can run successfully."""

from langchain_contrib.llms.fake import FakeLLM

from zamm.chains.general.get_dict import GetDictChain
from zamm.prompts.fake_parser import FakeDictParser


def test_multiple_outputs_can_run() -> None:
    """Test that GetMultipleOutputsChain can run successfully with multiple steps."""
    chain = GetDictChain(
        llm=FakeLLM(
            check_stops=True,
            sequenced_responses=["fake tool\n", "fake input\n"],
        ),
        prefix="Figure out what to do next.\n\n",
        variables={"tool": "Action", "tool_input": "Action Input"},
    )
    assert chain({}) == {
        "tool": "fake tool",
        "tool_input": "fake input",
    }


def test_multiple_outputs_can_run_in_one_step() -> None:
    """Test that GetMultipleOutputsChain can run successfully in a single step."""
    chain = GetDictChain(
        llm=FakeLLM(
            check_stops=True,
            sequenced_responses=["fake tool\nAction Input: fake input\nObservation:"],
        ),
        prefix="Figure out what to do next.\n\n",
        variables={"tool": "Action", "tool_input": "Action Input"},
        one_step=True,
        one_step_stop="Observation:",
        output_parser=FakeDictParser(),
    )
    assert chain({}) == {
        "tool": "Fake tool",
        "tool_input": "Fake input",
    }
