from typing import Any, Dict, List

from langchain.chains.base import Chain
from langchain.chains.llm import LLMChain
from langchain.chains.sequential import SequentialChain
from langchain.llms.base import BaseLLM
from pydantic import BaseModel

from zamm.chains.general.boolean_switch import BooleanSwitchChain
from zamm.prompts.chained import ChainedPromptTemplate
from zamm.prompts.prefixed import Prefix

from .filesystem import FileSystemTool
from .prompt import NEW_CONTENTS_PROMPT, REPLACE_CONTENTS_PROMPT, WHICH_FILE_PROMPT


class AskFileChain(LLMChain, BaseModel):
    """Asks LLM for path of file to edit."""

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables

    @property
    def output_keys(self) -> List[str]:
        return ["file_path"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        file_path = (
            self.llm(self.prompt.format(**inputs), stop=["`", "\n"]).strip().strip("`")
        )
        return {"file_path": file_path}


class FileOutputChain(LLMChain, BaseModel):
    """Asks LLM for content of new file."""

    @property
    def input_keys(self) -> List[str]:
        """Input keys to this chain."""
        return self.prompt.input_variables + ["file_path"]

    @property
    def output_keys(self) -> List[str]:
        return self.prompt.input_variables + ["new_contents"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        new_contents = self.llm(self.prompt.format(**inputs), stop=["```"]).strip("\n")
        FileSystemTool().write_file(inputs["file_path"], new_contents)
        return {**inputs, "new_contents": new_contents}


class ReadFileChain(Chain):
    @property
    def input_keys(self) -> List[str]:
        """Input keys this chain expects."""
        return ["file_path"]

    @property
    def output_keys(self) -> List[str]:
        """Output keys this chain expects."""
        return ["file_exists", "old_contents"]

    def _call(self, inputs: Dict[str, str]) -> Dict[str, str]:
        fs = FileSystemTool()
        return fs.read_file(file_path=inputs["file_path"]).as_llm_output(
            contents_key="old_contents"
        )


class EditFileChain(BooleanSwitchChain):
    def condition_was_met(self, outputs: Dict[str, Any]) -> bool:
        """Condition is whether or not the specified file exists."""
        return outputs["file_exists"]

    @classmethod
    def default(cls, llm: BaseLLM, prefix: Prefix):
        ask_file = AskFileChain(
            llm=llm, prompt=ChainedPromptTemplate("", prefix, WHICH_FILE_PROMPT)
        )
        read_file = ReadFileChain()
        pick_and_read_file = SequentialChain(
            chains=[ask_file, read_file],
            input_variables=ask_file.input_keys,
            return_all=True,
        )
        edit_file = FileOutputChain(
            llm=llm, prompt=ChainedPromptTemplate("", prefix, REPLACE_CONTENTS_PROMPT)
        )
        new_file = FileOutputChain(
            llm=llm, prompt=ChainedPromptTemplate("", prefix, NEW_CONTENTS_PROMPT)
        )
        return cls(
            option_picker=pick_and_read_file,
            true_chain=edit_file,
            false_chain=new_file,
        )
