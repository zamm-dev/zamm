# ZAMM

This is an informal automation tool where you show GPT how to do something, and have it do it for you afterwards. This is good for boring but straightforward tasks that you haven't gotten around to writing a proper script to automate.

We are entering a time when our target audiences may include machines as well as humans. As such, this tool will generate tutorials that you can edit to make pleasant for both humans and LLMs alike to read.

**This is an experimental tool, and has only been run on WSL Ubuntu so far.** It seems to work ok on the specific examples below. YMMV. Please feel free to add issues or PRs.

## Quickstart

`pipx` recommended over `pip` for install because it should allow you to run this with a different version of `langchain` than the one you might have installed:

```bash
pipx install zamm
```

Teach GPT to do something:

```bash
zamm teach
```

You will be roleplaying the LLM. The results of your interaction will be output as a Markdown tutorial file, which you can then edit to be more human-readable. See [this example](zamm/resources/tutorials/hello.md) of teaching the LLM how to create a "Hello world" script.

Afterwards, you can tell the LLM to do a slightly different task using that same tutorial:

```bash
zamm execute --task 'Write a script goodbye.sh that prints out "Goodbye world". Execute it.' --documentation zamm/resources/tutorials/hello.md
```

This results in [this example transcript](demos/hello-transcript.md) of LLM interactions. **Note that GPT successfully generalizes from the tutorial to code in a completely different language based just on the difference in filenames.** Imagine having to manually add that feature to a script!

### Using internal tutorials

Select any of the [prepackaged tutorials](zamm/resources/tutorials/) as documentation by prefacing their filename with `@internal`. The `.md` extension is optional.

For example:

```bash
zamm execute --task 'Protect the `main` branch' --documentation @internal/branch-protection
```

to protect the `main` branch of the project in the current directory on Github. (Note that this tutorial was written in mind for ZAMM-built projects, so YMMV for using this on custom projects.)

### Sessions

Sessions are recorded in case a crash happens, or if you want to change something up. On Linux, sessions are saved to `~/.local/share/zamm/sessions/`. To continue from the most recent session, run

```bash
zamm teach --last-session
```

### Free-styling

You can also simply tell the LLM to do something without teaching it to do so beforehand. However, this is a lot more brittle. An example of a free-style command that works:

```bash
zamm execute --task 'Write a script hello.py that prints out "Hello world". Execute it.'
```

The resulting transcript can be found [here](demos/freestyle-hello-transcript.md).

## Prompting

When a step is failing and you need faster iteration by repeatedly testing a single prompt, you can do so with the `prompt` command. First, write your prompt out to a file on disk. Then run this command:

```bash
zamm prompt --stop '\n' --raw <path-to-prompt>
```
