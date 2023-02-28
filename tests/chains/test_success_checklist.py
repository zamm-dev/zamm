import langchain_visualizer  # isort:skip  # noqa: F401

import vcr_langchain as vcr
from langchain.llms import OpenAI

from zamm.chains.success_criteria import GetSuccessCriteriaChain

DOCUMENTATION_PATH = "tests/resources/poetry.zamm.md"

with open(DOCUMENTATION_PATH) as f:
    documentation = f.read()


@vcr.use_cassette()
async def test_success_checklist():
    llm = OpenAI(temperature=0)
    results = GetSuccessCriteriaChain(llm=llm)(
        {
            "documentation": documentation,
            "task": "Start a new Python project called ZAMM",
        },
        return_only_outputs=False,
    )
    checklist = results["checklist"]
    assert (
        "Verify that the output looks like `Created package ZAMM in ZAMM`" in checklist
    )
    assert "Verify that a folder named `ZAMM` is created" in checklist


if __name__ == "__main__":
    langchain_visualizer.visualize(test_success_checklist)
