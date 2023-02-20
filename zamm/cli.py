import langchain_visualizer  # isort:skip  # noqa: F401
import asyncio
import os
import sys
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
    llm: BaseLLM,
    task: str,
    tutorial: Optional[str],
    cassette_path: str,
):
    async def run():
        employee = ZammEmployee(llm=llm, terminal_safe_mode=True)
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


@app.command()
def teach(
    session_recording: Optional[typer.FileText] = SESSION_RECORD_OPTION,
    output: Optional[typer.FileTextWrite] = OUTPUT_OPTION,
):
    """Record a tutorial interaction."""

    cassette_path, output_path = get_cassette_and_output(
        cassette=session_recording, output=output
    )

    llm = Human()
    if session_recording is not None:
        session_recording.close()
    play_interactions(
        llm=llm, cassette_path=cassette_path, tutorial_output_path=output_path
    )


@app.command()
def re_record(
    session_recording: typer.FileText = typer.Option(
        ...,
        help="Recorded interactions from a previous unfinished session.",
    ),
    output: Optional[typer.FileTextWrite] = OUTPUT_OPTION,
):
    """Re-record a tutorial interaction.

    Keep all inputs the same. Useful for when you're making cosmetic changes to the
    prompting, but wish to otherwise keep everything the same.
    """
    _, output_path = get_cassette_and_output(cassette=session_recording, output=output)

    try:
        interactions = yaml.load(session_recording, Loader=yaml.Loader)["interactions"]
        session_recording.close()
        inputs = [
            i["response"]
            for i in interactions
            if i["request"]["uri"].startswith("tool://Human")
        ]
        llm = Human(prerecorded_responses=inputs)
        os.remove(session_recording.name)
        play_interactions(llm, session_recording.name, tutorial_output_path=output_path)
    except yaml.YAMLError as exc:
        print(exc)


@app.command()
def execute(
    task: str = typer.Option(
        ...,
        help="What you'd like the LLM to do",
    ),
    documentation: Optional[typer.FileText] = typer.Option(
        None,
        help="Documentation file to help the LLM accomplish the task",
    ),
    session_recording: Optional[typer.FileText] = SESSION_RECORD_OPTION,
):
    """Ask the LLM to do something."""

    cassette_path = get_cassette_path(cassette=session_recording)

    llm = OpenAI(model_name="text-davinci-003", temperature=0)
    if session_recording is not None:
        session_recording.close()
    if documentation is None:
        tutorial = None
    else:
        tutorial = documentation.read()
        documentation.close()
    execute_llm_task(llm=llm, task=task, tutorial=tutorial, cassette_path=cassette_path)
