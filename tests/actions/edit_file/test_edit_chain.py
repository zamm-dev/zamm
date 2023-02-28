import os
from contextlib import contextmanager

from langchain.prompts.prompt import PromptTemplate

from zamm.actions.edit_file import FileOutputChain
from zamm.llms.fake import FakeLLM


@contextmanager
def temporary_file(path: str):
    if os.path.isfile(path):
        os.remove(path)
    assert not os.path.isfile(path)

    try:
        yield
        assert os.path.isfile(path)
    finally:
        # remove it for future testing
        os.remove(path)


def test_write_tabs_to_file():
    file_path = "tests/resources/tab_write.txt"
    contents = "\ta"
    with temporary_file(file_path):
        FileOutputChain(
            llm=FakeLLM(sequenced_responses=["\ta"]),
            prompt=PromptTemplate(
                input_variables=["file_path", "file_contents"],
                template="Dummy {file_path} with {file_contents}",
            ),
        )(
            {
                "file_path": file_path,
                "file_contents": contents,
            }
        )
        with open(file_path) as f:
            assert f.read() == contents
