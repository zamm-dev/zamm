# Creating a private custom Makefile

Say you want to do the following task:

> Setup a custom private Makefile that applies generally

This is sometimes useful because other project maintainers might not like you polluting their Makefiles with your own custom commands, but you still want access to shortcut commands that help your own workflow out a lot. You can do this by following these steps:

## Defining the custom Makefile

Let's first create the private Makefile that we want to apply every time we run the `make` command. Edit `~/.local/share/zamm/makefiles/general.mk` to contain:

```
.PHONY: update-main

CURRENT_BRANCH := $(shell git branch --show-current)

update-main:
	git checkout main
	git pull

new-branch: update-main
	git checkout -b $(NAME)

sync-main: update-main
	git checkout $(CURRENT_BRANCH)
	git rebase main
```

## Defining the custom `make` command

Next, we will have to create a Bash script that calls `make` on any existing Makefile in the current directory, as well as this new private Makefile. To do that, we first have to figure out where the existing `make` command is located:

```bash
$ which make
/usr/bin/make
```

Now we'll create our replacement `make` script that calls out to **/usr/bin/make**. We'll put the script at `~/.local/share/zamm/bin/make` with the contents:

```
#!/bin/bash
/usr/bin/make -f Makefile -f ~/.local/share/zamm/makefiles/general.mk "$@"
```

And make it executable:

```bash
$ chmod +x ~/.local/share/zamm/bin/make
```

## Adding custom `make` to the `PATH`

Finally, we have to make this new `make` command the default. Let's see which shell you are using so that we know which shell init file to edit.

```bash
$ echo $SHELL
/usr/bin/zsh
```

From this we see that you are using the **ZSH** shell, which is initialized by the `.zshrc` file in your home directory. Now we add that `make` script we created to your `PATH` when initializing `.zshrc`.

```bash
$ echo "export PATH=~/.local/share/zamm/bin:\$PATH" >> ~/.zshrc
```

## Confirmation

Finally, let's confirm that the new script is overriding the old `make`:

```bash
$ /usr/bin/zsh -c "source ~/.zshrc; which make"
/home/amos/.local/share/zamm/bin/make
```

That's all! **Don't take any more steps** because the task is now done!