import os
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

    def read_file(self, file_path: str) -> FileRead:
        if os.path.isfile(file_path):
            with open(file_path) as f:
                return FileRead(file_exists=True, contents=f.read())
        return FileRead(file_exists=False)

    def write_file(self, file_path: str, contents: str) -> bool:
        folder = os.path.dirname(file_path)
        if folder != "":
            os.makedirs(folder, exist_ok=True)
        with open(file_path, "w") as f:
            f.write(contents)
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
