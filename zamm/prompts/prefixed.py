from typing import Any, List, Optional, Union

from langchain.prompts import PromptTemplate
from langchain.prompts.base import StringPromptTemplate

Prefix = Union[str, StringPromptTemplate]


class PrefixedPromptTemplate(StringPromptTemplate):
    prefix: StringPromptTemplate

    def __init__(
        self,
        prefix: Prefix,
        input_variables: Optional[List[str]] = None,
        **kwargs: Any,
    ):
        if isinstance(prefix, str):
            prefix_input_vars = input_variables or []
            prefix = PromptTemplate(input_variables=prefix_input_vars, template=prefix)
        elif input_variables is not None:
            raise ValueError("Cannot both prompt template and custom input_variables")
        super().__init__(
            prefix=prefix,
            input_variables=prefix.input_variables,
            **kwargs,
        )
