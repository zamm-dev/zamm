import os
import re
import warnings
from contextlib import contextmanager
from importlib import resources
from typing import Any, Dict, List, Optional, Union

from fvalues import F
from langchain.chains.base import Chain
from langchain.prompts import BasePromptTemplate

INTERNAL_TUTORIAL_PREFIX = "@internal"
INTERNAL_TUTORIAL_PACKAGE = "zamm.resources.tutorials"
INTERNAL_TUTORIAL_PATH = re.compile("^/zamm/resources/tutorials")


def safe_inputs(
    lc_object: Union[Chain, BasePromptTemplate], inputs: Dict[str, str]
) -> Dict[str, str]:
    if isinstance(lc_object, Chain):
        input_variables = lc_object.input_keys
    elif isinstance(lc_object, BasePromptTemplate):
        input_variables = lc_object.input_variables
    else:
        raise ValueError(f"Unknown type for {lc_object}")
    return {k: v for k, v in inputs.items() if k in input_variables}


def safe_call(chain: Chain, inputs: Dict[str, str]) -> Dict[str, Any]:
    return chain(safe_inputs(chain, inputs))


def safe_format(template: BasePromptTemplate, inputs: Dict[str, str]) -> str:
    return template.format(**safe_inputs(template, inputs))


def f_join(joiner: str, substrings: List[Union[str, F]]) -> F:
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
    # not really ANSI escape sequence, but similar vibe
    rs = r"\r|\\r"

    # how we know we're in an ANSI escape sequence
    escape_prefixes = [r"\\033", r"\\e", r"\x1b"]
    brackets = [r"\[", r"\("]
    all_ansi_escapes = [
        prefix + bracket for prefix in escape_prefixes for bracket in brackets
    ]
    all_escapes_regex = "|".join(all_ansi_escapes)

    # the actual command once we know there's an escape
    escape_command = r"[\d;]*m|\d*[A-K]"

    escapes = f"({all_escapes_regex})({escape_command})"
    return re.compile("|".join([rs, escapes]))


def remove_match(line: str, next_match: re.Match[str]):
    return line[: next_match.start()] + line[next_match.end() :]


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
            if full_match == "\r" or full_match == "\\r":
                line = line[next_match.end() :]
            elif command.endswith("A"):
                num_lines = int(command[:-1])
                cleaned = cleaned[:-num_lines]
                line = line[next_match.end() :]
            elif command.endswith("m"):
                line = remove_match(line, next_match)
            elif command.endswith("J"):
                if command == "0J":
                    # todo: track cursor position properly
                    # do nothing because it only affects things after the cursor
                    pass
                line = remove_match(line, next_match)
            elif command.endswith("K"):
                if command == "K" or command == "0K":
                    # erase after cursor. no-op because not tracking cursor
                    line = remove_match(line, next_match)
                else:
                    line = line[next_match.end() :]
            else:  # just remove the ANSI escape code if we can't interpret it
                line = remove_match(line, next_match)
            next_match = re.search(regex, line)
        cleaned.append(line)
    return "\n".join(cleaned)


def read_documentation(documentation: str) -> str:
    """Read documentation from a potentially internal path"""
    documentation = re.sub(
        INTERNAL_TUTORIAL_PATH, INTERNAL_TUTORIAL_PREFIX, documentation
    )
    if documentation.startswith(INTERNAL_TUTORIAL_PREFIX):
        internal_path = documentation[len(INTERNAL_TUTORIAL_PREFIX) + 1 :]
        if not internal_path.endswith(".md"):
            internal_path += ".md"
        package_dirs = internal_path.split("/")
        internal_package_cont = ".".join(package_dirs[:-1])
        if internal_package_cont == "":
            full_package = INTERNAL_TUTORIAL_PACKAGE
        else:
            full_package = f"{INTERNAL_TUTORIAL_PACKAGE}.{internal_package_cont}"
        return resources.read_text(full_package, package_dirs[-1])

    with open(documentation) as f:
        return f.read()


def get_stop_hit(input: str, stops: List[str]) -> Optional[str]:
    """Find out which stop, if any, have been triggered."""
    for stop in stops:
        if stop in input:
            return stop
    return None


def artificial_stop(
    input: str, stops: List[str], stop_hit: Optional[str] = None
) -> str:
    if stop_hit is None:
        stop_hit = get_stop_hit(input, stops)
        assert stop_hit is not None, f"No stop from {stops} found in {input}"
    separated_inputs = [x for x in input.split(stop_hit) if x]
    if separated_inputs == []:
        result = ""
    else:
        result = separated_inputs[0]
        if len(separated_inputs) > 1:
            rest = stop_hit.join(separated_inputs[1:])
            warnings.warn(
                f"Input: '{result}'\nIgnoring rest of input after stop: '{rest}'"
            )

    return result


@contextmanager
def current_directory(path: str):
    og_cwd = os.getcwd()
    os.makedirs(path, exist_ok=True)
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(og_cwd)


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
