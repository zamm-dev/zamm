import os
import re
from contextlib import contextmanager
from typing import Any

from fvalues import F
from langchain.chains.base import Chain
from langchain.prompts import BasePromptTemplate


def safe_inputs(
    lc_object: Chain | BasePromptTemplate, inputs: dict[str, str]
) -> dict[str, str]:
    if isinstance(lc_object, Chain):
        input_variables = lc_object.input_keys
    elif isinstance(lc_object, BasePromptTemplate):
        input_variables = lc_object.input_variables
    else:
        raise ValueError(f"Unknown type for {lc_object}")
    return {k: v for k, v in inputs.items() if k in input_variables}


def safe_call(chain: Chain, inputs: dict[str, str]) -> dict[str, Any]:
    return chain(safe_inputs(chain, inputs))


def safe_format(template: BasePromptTemplate, inputs: dict[str, str]) -> str:
    return template.format(**safe_inputs(template, inputs))


def f_join(joiner: str, substrings: list[str | F]) -> F:
    if substrings == []:
        return F("")

    joined = ""
    parts = []
    for substring in substrings:
        joined += substring
        if isinstance(substring, str):
            parts.append(substring)
        else:
            parts.extend(list(substring.parts))

        # if it's the empty string, we can just avoid polluting parts with it
        if joiner != "":
            joined += joiner
            parts.append(joiner)

    if joiner != "":  # joiner should only exist in between parts
        joined = joined[: -len(joiner)]
        parts.pop()

    return F(joined, parts=tuple(parts))


def ansi_escape_regex():
    escape_command = r"\[([\d;]*m|\d[A-K])"
    rs = r"\r|\\r"
    escape_prefixes = "|".join([r"\\033", r"\\e", r"\x1b"])
    escapes = f"({escape_prefixes}){escape_command}"
    return re.compile("|".join([rs, escapes]))


def remove_ansi_escapes(input):
    """ANSI escape remover.

    Incomplete escape interpreter based on
    https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
    """
    regex = ansi_escape_regex()
    cleaned = []
    for line in input.split("\n"):
        next_match = re.search(regex, line)
        while next_match is not None:
            full_match = next_match.group()
            _, command = next_match.groups()
            if full_match == "\r" or full_match == "\\r" or command.endswith("K"):
                line = line[next_match.end() :]
            elif command.endswith("A"):
                num_lines = int(command[:-1])
                cleaned = cleaned[:-num_lines]
                line = line[next_match.end() :]
            else:  # just remove the ANSI escape code if we can't interpret it
                line = line[: next_match.start()] + line[next_match.end() :]
            next_match = re.search(regex, line)
        cleaned.append(line)
    return "\n".join(cleaned)


@contextmanager
def current_directory(path: str):
    og_cwd = os.getcwd()
    os.makedirs(path, exist_ok=True)
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(og_cwd)
