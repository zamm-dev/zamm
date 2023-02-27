import os
import re
from dataclasses import dataclass
from typing import Any, Callable, Dict, Optional

from pydantic import BaseModel
from vcr.cassette import Cassette
from vcr_langchain.patch import GenericPatch, add_patchers


@dataclass
class FileRead:
    file_exists: bool
    contents: Optional[str] = None

    def as_llm_output(self, contents_key: str = "contents") -> Dict[str, Any]:
        output: Dict[str, Any] = {"file_exists": self.file_exists}
        if self.file_exists:
            output[contents_key] = self.contents
        else:
            output[contents_key] = None
        return output


class FileSystemTool(BaseModel):
    """Tool for file system interactions."""

    spaces_in_tab = 8
    """If there's this many spaces at the beginning of the line, it gets treated as a
    tab.
    """

    def interpret_path(self, file_path: str) -> str:
        """Makes sure that ~ gets interpreted as home directory instead of filename."""
        return os.path.expanduser(file_path)

    @property
    def tab_regex(self):
        return re.compile(f"^ {{{self.spaces_in_tab}}}")

    def _spaces_to_tabs_per_line(self, line: str) -> str:
        while re.search(self.tab_regex, line) is not None:
            line = re.sub(self.tab_regex, "\t", line)
        return line

    def spaces_to_tabs(self, input: str) -> str:
        lines = input.split("\n")
        tabbed_lines = [self._spaces_to_tabs_per_line(line) for line in lines]
        return "\n".join(tabbed_lines)

    def should_convert_spaces_to_tabs(self, filename: str) -> bool:
        return filename == "Makefile" or filename.endswith(".mk")

    def read_file(self, file_path: str) -> FileRead:
        file_path = self.interpret_path(file_path)
        if os.path.isfile(file_path):
            with open(file_path) as f:
                return FileRead(file_exists=True, contents=f.read())
        return FileRead(file_exists=False)

    def write_file(self, file_path: str, contents: str) -> bool:
        file_path = self.interpret_path(file_path)
        folder = os.path.dirname(file_path)
        if folder != "":
            os.makedirs(folder, exist_ok=True)

        if self.should_convert_spaces_to_tabs(file_path):
            tabbed_contents = self.spaces_to_tabs(contents)
        else:
            tabbed_contents = contents

        with open(file_path, "w") as f:
            f.write(tabbed_contents)

        return True


class FileReadPatch(GenericPatch):
    def __init__(self, cassette: Cassette):
        super().__init__(cassette, FileSystemTool, "read_file")

    def get_same_signature_override(self) -> Callable:
        def _call(og_self: FileSystemTool, file_path: str) -> FileRead:
            """Same signature override patched into FileSystemTool"""
            return self.generic_override(og_self, file_path=file_path)

        return _call


class FileWritePatch(GenericPatch):
    def __init__(self, cassette: Cassette):
        super().__init__(cassette, FileSystemTool, "write_file")

    def get_same_signature_override(self) -> Callable:
        def _call(og_self: FileSystemTool, file_path: str, contents: str) -> FileRead:
            """Same signature override patched into FileSystemTool"""
            return self.generic_override(
                og_self, file_path=file_path, contents=contents
            )

        return _call


add_patchers(FileReadPatch, FileWritePatch)
