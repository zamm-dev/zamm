# ZAMM

Teach GPT how to do something, and have it do it for you afterwards. This is good for boring but straightforward tasks that you haven't gotten around to writing a proper script to automate.

We are entering a time when our target audiences may include machines as well as humans. As such, this tool will generate tutorials that you can edit to make pleasant for both humans and LLMs alike to read.

**This is an experimental tool, and has only been run on WSL Ubuntu so far.** It seems to work ok on the specific examples below. YMMV. Please feel free to add issues or PRs.

## Quickstart

Teach GPT to do something:

```bash
zamm teach
```

You will be roleplaying the LLM. Sessions are recorded in case a crash happens, or if you want to change something up. On Linux, sessions are saved to `~/.local/share/zamm/sessions/`. To continue from a previous session, run:

```bash
zamm teach --session-recording <path-to-recording>
```

Once a session finishes, you can use the newly generated tutorial file to do something. For example, to run the recording I made at [zamm/resources/tutorials/hello.md](zamm/resources/tutorials/hello.md):

```bash
zamm execute --task 'Write a script goodbye.sh that prints out "Goodbye world". Execute it.' --documentation zamm/resources/tutorials/hello.md
```

**Note that GPT successfully generalizes from the tutorial to code in a completely different language based just on the difference in filenames.** Imagine having to manually add that feature to a script!

### Free-styling

You can also simply tell the LLM to do something without teaching it to do so beforehand. However, this is a lot more brittle. An example of a free-style command that works:

```bash
zamm execute --task 'Write a script hello.py that prints out "Hello world". Execute it.'
```
