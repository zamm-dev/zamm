from langchain_contrib.llms.fake import FakeLLM

from zamm.actions.use_terminal import UseTerminal, ZTerminal
from zamm.utils import temporary_file


def test_edit_file_redirect():
    with temporary_file("tests/resources/redirect.txt"):
        llm = FakeLLM(sequenced_responses=["nano tests/resources/redirect.txt", "Blah"])
        terminal_action = UseTerminal.default(
            llm=llm, prefix="Dummy prefix", terminal=ZTerminal()
        )
        assert terminal_action.use({}) == {
            "file_path": "tests/resources/redirect.txt",
            "file_exists": False,
            "old_contents": None,
            "new_contents": "Blah",
        }
