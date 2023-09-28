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

## Enabling mouse support

Put this in your `~/.tmux.conf` file:

```
set -g mouse on
```

As noted in [this answer](https://unix.stackexchange.com/a/559562), you may have to restart your tmux instance if you're already running it. You can restart it with the instructions as noted, or if you're only running the one session, `ctrl-B` and then type `:kill-session<RET>`. Other commands are noted [here](https://www.baeldung.com/linux/tmux-kill-respawn-pane).

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

## VSCode

Note that if you're using a remote VSCode session, xclip won't transfer into your own OS's clipboard. In this case, you may want to open a native VSCode pane instead while leaving the others running on tmux.
