from langchain.agents.load_tools import _BASE_TOOLS

from .load_tools import _get_files_read, _get_files_write

_BASE_TOOLS["file-reader"] = _get_files_read
_BASE_TOOLS["file-writer"] = _get_files_write
