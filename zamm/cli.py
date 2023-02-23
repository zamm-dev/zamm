import langchain_visualizer  # isort:skip  # noqa: F401
import asyncio
import glob
import os
import sys
from enum import Enum
from importlib import resources
from typing import Callable, Optional

import typer
import ulid
import vcr_langchain as vcr
import yaml
from appdirs import AppDirs
from langchain.llms.base import BaseLLM
from langchain.llms.openai import OpenAI
from vcr.record_mode import RecordMode

from zamm.agents.employee import ZammEmployee
from zamm.chains.ask_task import AskForTaskChain
from zamm.llms.human import Human
from zamm.utils import current_directory

DOCUMENTATION_PATH = "documentation.zamm.md"
VISUALIZE = True
app_dirs = AppDirs(appname="zamm")
ZAMM_SESSION_PATH = app_dirs.user_data_dir + "/sessions"
INTERNAL_TUTORIAL_PREFIX = "@internal"
INTERNAL_TUTORIAL_PACKAGE = "zamm.resources.tutorials"


app = typer.Typer(pretty_exceptions_show_locals=False)


def run_chain(cassette_path: str, run: Callable):
    # todo: change ICE to allow for running from other modules
    run.__module__ = "__main__"

    with vcr.use_cassette(
        path=cassette_path,
        record_mode=RecordMode.NEW_EPISODES,
    ):
        with current_directory(os.getcwd()):
            try:
                if VISUALIZE:
                    sys.argv = sys.argv[:1]
                    return langchain_visualizer.visualize(run)
                else:
                    return asyncio.get_event_loop().run_until_complete(run())
            except RuntimeError:
                print(f"Exiting, session recording should be saved to {cassette_path}")


def play_interactions(
    llm: BaseLLM,
    cassette_path: str,
    tutorial_output_path: str,
):
    async def run():
        task = AskForTaskChain(llm=llm)({})
        # always trust shell commands from the person who invoked this program in the
        # first place
        employee = ZammEmployee(llm=llm, terminal_safe_mode=False)
        result = employee(task)
        if result is not None:
            with open(tutorial_output_path, "w") as f:
                try:
                    f.write(result["output"])
                    print(f"Tutorial saved to {tutorial_output_path}")
                except Exception as e:
                    print(f"Could not save tutorial to {tutorial_output_path}: {e}")

    run_chain(cassette_path, run)


def execute_llm_task(
    employee: ZammEmployee,
    task: str,
    tutorial: Optional[str],
    cassette_path: str,
):
    async def run():
        args = {"task": task}
        if tutorial is not None:
            args["documentation"] = tutorial
        result = employee(args)
        if result is not None:
            print("LLM indicates it has completed the task")

    run_chain(cassette_path, run)


SESSION_RECORD_OPTION = typer.Option(
    None,
    help="Recorded interactions from a previous unfinished session.",
)

OUTPUT_OPTION = typer.Option(
    None,
    help="File to output the tutorial to.",
)


def get_cassette_path(cassette: Optional[typer.FileText]):
    if cassette is None:
        os.makedirs(ZAMM_SESSION_PATH, exist_ok=True)
        return f"{ZAMM_SESSION_PATH}/session_{ulid.new()}.yaml"
    else:
        cassette.close()
        return cassette.name


def get_cassette_and_output(
    cassette: Optional[typer.FileText],
    output: Optional[typer.FileTextWrite],
):
    cassette_path = get_cassette_path(cassette)

    if output is None:
        session_prefix = os.path.splitext(cassette_path)[0]
        output_path = f"{session_prefix}.md"
    else:
        output_path = output.name
        output.close()

    return cassette_path, output_path


def get_last_session():
    # https://stackoverflow.com/a/39327156
    files = glob.glob(f"{ZAMM_SESSION_PATH}/session_*.yaml")
    if len(files) == 0:
        print("No sessions found.")
        sys.exit(1)
    return max(files, key=os.path.getctime)


@app.command()
def teach(
    session_recording: Optional[typer.FileText] = SESSION_RECORD_OPTION,
    last_session: bool = typer.Option(
        False,
        help="The last session that was in progress",
    ),
    output: Optional[typer.FileTextWrite] = OUTPUT_OPTION,
):
    """Record a tutorial interaction."""

    cassette_path, output_path = get_cassette_and_output(
        cassette=session_recording, output=output
    )

    if last_session:
        cassette_path = get_last_session()

    llm = Human()
    play_interactions(
        llm=llm, cassette_path=cassette_path, tutorial_output_path=output_path
    )


@app.command()
def re_record(
    session_recording: Optional[typer.FileText] = typer.Option(
        None,
        help="Recorded interactions from a previous unfinished session.",
    ),
    last_session: bool = typer.Option(
        False,
        help="The last session that was in progress",
    ),
    output: Optional[typer.FileTextWrite] = OUTPUT_OPTION,
):
    """Re-record a tutorial interaction.

    Keep all inputs the same. Useful for when you're making cosmetic changes to the
    prompting, but wish to otherwise keep everything the same.
    """
    if not session_recording and not last_session:
        print("You must either specify session-recording or use --last-session.")
        sys.exit(1)

    cassette_path, output_path = get_cassette_and_output(
        cassette=session_recording, output=output
    )

    if last_session:
        cassette_path = get_last_session()

    try:
        with open(cassette_path) as c:
            interactions = yaml.load(c, Loader=yaml.Loader)["interactions"]
        inputs = [
            i["response"]
            for i in interactions
            if i["request"]["uri"].startswith("tool://Human")
        ]
        llm = Human(prerecorded_responses=inputs)
        os.remove(cassette_path)
        play_interactions(llm, cassette_path, tutorial_output_path=output_path)
    except yaml.YAMLError as exc:
        print(exc)


class Safety(str, Enum):
    # todo: use Python's Flag instead
    on = "on"
    off = "off"


@app.command()
def execute(
    task: str = typer.Option(
        ...,
        help="What you'd like the LLM to do",
    ),
    documentation: Optional[str] = typer.Option(
        None,
        help=(
            "Documentation file to help the LLM accomplish the task. Prefix with "
            "@internal for internal help files."
        ),
    ),
    session_recording: Optional[typer.FileText] = SESSION_RECORD_OPTION,
    model: str = typer.Option(
        "text-davinci-003",
        help="What OpenAI large language model to use for execution",
    ),
    safety: Safety = typer.Option(
        Safety.on,
        help=(
            "If on, will ask user to confirm every terminal command. If off, will run "
            "LLM commands automatically, WHICH MAY EXPOSE YOU TO LLM ATTACKS."
        ),
    ),
):
    """Ask the LLM to do something."""

    cassette_path = get_cassette_path(cassette=session_recording)

    llm = OpenAI(model_name=model, temperature=0, max_tokens=-1)
    if session_recording is not None:
        session_recording.close()
    if documentation is None:
        tutorial = None
    else:
        if documentation.startswith(INTERNAL_TUTORIAL_PREFIX):
            internal_path = documentation[len(INTERNAL_TUTORIAL_PREFIX) + 1 :]
            if not internal_path.endswith(".md"):
                internal_path += ".md"
            tutorial = resources.read_text(INTERNAL_TUTORIAL_PACKAGE, internal_path)
        else:
            with open(documentation) as f:
                tutorial = f.read()
    employee = ZammEmployee(llm=llm, terminal_safe_mode=safety.value == "on")
    execute_llm_task(
        employee=employee, task=task, tutorial=tutorial, cassette_path=cassette_path
    )
