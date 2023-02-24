import vcr_langchain as vcr
from langchain.llms import OpenAI

from zamm.agents.employee import ZammEmployee


def test_execute_goodbye_task():
    with open("zamm/resources/tutorials/hello.md") as tutorial:
        with vcr.use_cassette("tests/resources/goodbye.yaml"):
            llm = OpenAI(model_name="text-davinci-003", temperature=0, max_tokens=-1)
            employee = ZammEmployee(llm=llm, terminal_safe_mode=True)
            results = employee(
                {
                    "task": (
                        'Write a script goodbye.sh that prints out "Goodbye world". '
                        "Execute it."
                    ),
                    "documentation": tutorial.read(),
                }
            )
            assert results is not None
