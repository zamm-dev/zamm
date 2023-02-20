import os

from zamm.agents.tools.files import FilesTool

tool = FilesTool()


class TemporaryFile:
    def __init__(self, path):
        self.path = path

    def __enter__(self):
        if os.path.isfile(self.path):
            os.remove(self.path)
            assert not os.path.isfile(self.path)

    def __exit__(self, *_):
        try:
            # check that cassette was successfully written to
            assert os.path.isfile(self.path)
        finally:
            # remove it for future testing
            os.remove(self.path)


def test_read_nonexistent_file():
    assert tool.read("tests/resources/nonexistent.py") == (
        "Unfortunately, the file at `tests/resources/nonexistent.py` does not exist."
    )


def test_read_real_file():
    assert (
        tool.read("tests/resources/existent.py")
        == """
The contents of the file at `tests/resources/existent.py` are

~~~~~~~~~~
print("Hello world")

~~~~~~~~~~
""".strip()
    )


def test_write_file_success():
    path = "tests/resources/writing_test.py"
    contents = 'print("Goodbye world")'
    with TemporaryFile(path):
        assert tool.write(path, contents) == (
            "The file at `tests/resources/writing_test.py` was successfully written to."
        )
        with open(path) as f:
            assert f.read() == contents


def test_write_file_failure():
    path = "tests/resources/folder/"
    contents = 'print("Goodbye world")'
    assert tool.write(path, contents) == (
        "The file at `tests/resources/folder/` could not be written to: "
        "[Errno 21] Is a directory: 'tests/resources/folder/'"
    )
