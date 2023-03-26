from langchain_contrib.llms.testing import FakeLLM
from langchain_contrib.tools.terminal import TerminalToolChain

from zamm.actions.use_terminal import UseTerminal
from zamm.utils import temporary_file


def test_edit_file_redirect():
    with temporary_file("tests/resources/redirect.txt"):
        llm = FakeLLM(sequenced_responses=["nano tests/resources/redirect.txt", "Blah"])
        terminal_action = UseTerminal.default(
            llm=llm, prefix="Dummy prefix", terminal_chain=TerminalToolChain()
        )
        assert terminal_action.use({}) == {
            "file_path": "tests/resources/redirect.txt",
            "file_exists": "False",
            "new_contents": "Blah",
        }
