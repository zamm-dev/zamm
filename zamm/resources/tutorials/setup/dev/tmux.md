# Setting up tmux for a project

If there are commands you need to run often every time you open a project, you may want to set up a tmux session to run them for you.

```sh
#!/bin/bash

tmux new-session -d -s langchain-contrib 'cd docs && poetry run /usr/bin/make livehtml'
tmux split-window -v 'poetry run jupyter-lab'
tmux split-window -v 'poetry shell'
tmux attach-session -t langchain-contrib
```

Or for Tauri:

```sh
#!/bin/bash

tmux new-session -d -s zamm 'yarn workspace gui storybook --ci'
tmux split-window -v 'yarn tauri dev'
tmux split-window -v
tmux attach-session -t zamm
```

where `split-window` without any command argument simply opens a new shell. To split your screen horizontally instead of vertically, use `-h` instead of `-v`.

Then

```bash
$ chmod +x tmux.sh
```

You may want to first check if the session exists already, and if so simply attach to it:

```sh
#!/bin/bash

SESSION_NAME="zamm"

tmux has-session -t $SESSION_NAME

if [ $? != 0 ]; then
    tmux new-session -d -s $SESSION_NAME 'yarn workspace gui storybook --ci'
    tmux split-window -h 'yarn tauri dev'
fi

tmux attach-session -t $SESSION_NAME

```

## Enabling mouse support

Put this in your `~/.tmux.conf` file:

```
set -g mouse on
```

As noted in [this answer](https://unix.stackexchange.com/a/559562), you may have to restart your tmux instance if you're already running it. You can restart it with the instructions as noted, or if you're only running the one session, `ctrl-B` and then type `:kill-session<RET>`. Other commands are noted [here](https://www.baeldung.com/linux/tmux-kill-respawn-pane):

- To forcibly restart a pane, `ctrl-B :respawn-pane -k`

Now you can simply click on a pane with the mouse to move to it.

## Copy-pasting with mouse

Install [Tmux Plugin Manager](https://github.com/tmux-plugins/tpm):

```bash
$ git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm
```

Then add this to the bottom of `tmux.conf`:

```
run '~/.tmux/plugins/tpm/tpm'
```

Then we install [tmux-yank](https://github.com/tmux-plugins/tmux-yank) as a plugin. Add `set -g @plugin 'tmux-plugins/tmux-yank'` before that line in `tmux.conf`.

As noted in [this question](https://unix.stackexchange.com/questions/318281/how-to-copy-and-paste-with-a-mouse-with-tmux), we may have to install `xclip`. You can check that this is working with:

```bash
$ xclip -o
<pasted text here>
```

## Keeping panes open after commands exit

To keep panes open after their commands exit (e.g. to easily respawn those commands, or to see what caused them to die), there are several options as noted [here](https://unix.stackexchange.com/questions/17116/prevent-pane-window-from-closing-when-command-completes-tmux). We'll choose to add this to `~/.tmux.conf`, as it allows us to quickly respawn panes with the commands they were created with:

```
set-option -g remain-on-exit on
```

To run a command without letting it exit after completion (e.g. to cd into a directory), run the command

```bash
$ tmux send-keys -t $SESSION_NAME:1.1 'cd src-svelte' Enter
```

## Resizing split panes to be equal

If you're creating multiple panes and want them all to have the same size, follow the instructions [here](https://unix.stackexchange.com/a/37754).

To do this with the commandline:

```bash
$ tmux select-layout -t $SESSION_NAME:1 even-horizontal
```

## Renaming windows

See [here](https://stackoverflow.com/a/40333995).

## Shortcuts

To make it easy to restart a pane, for example, edit `~/.tmux.conf`:

```
bind r respawn-pane -k
```

Then type `<ctrl-B>:source-file ~/.tmux.conf` to read in the new configuration.

## VSCode

Note that if you're using a remote VSCode session, xclip won't transfer into your own OS's clipboard. In this case, you may want to open a native VSCode pane instead while leaving the others running on tmux.
