from langchain.agents.tools import Tool

from zamm.agents.tools.files import FilesTool


def _get_files_read() -> Tool:
    return Tool(
        "File Reader",
        FilesTool().read,
        (
            "Use this to read files. Input should be the file path. Output will be a "
            "description of the file, including the file contents."
        ),
    )


def _get_files_write() -> Tool:
    return Tool(
        "File Writer",
        FilesTool().write,  # type: ignore
        (
            "Use this to write files. Inputs should be the file path and the content "
            "to be written to the file. Output will be a description of the results "
            "of the write."
        ),
    )
