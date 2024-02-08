# Setting up Git

Git probably exists but is not yet configured. To do so, ask the user for their name and email, and run the commands accordingly:

```bash
$ git config --global user.name "Amos Jun-yeung Ng"
$ git config --global user.email "me@amos.ng"
```

Let's then set the default branch name to "main":

```bash
$ git config --global init.defaultBranch main
```

This avoids giving us warnings like this one

```
$ git init
hint: Using 'master' as the name for the initial branch. This default branch name
hint: is subject to change. To configure the initial branch name to use in all
hint: of your new repositories, which will suppress this warning, call:
hint: 
hint:   git config --global init.defaultBranch <name>
hint: 
hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
hint: 'development'. The just-created branch can be renamed via this command:
hint: 
hint:   git branch -m <name>
Initialized empty Git repository in /home/amos/projects/take-home/.git/
```

## Setting up SSH

To set up SSH for GitHub SSH access, we first generate SSH public-private keypairs:

```bash
$ ssh-keygen
```

Then, we add it to GitHub by going to [https://github.com/settings/keys](https://github.com/settings/keys) and clicking on "New SSH key". If you see a login page, then you should login first.

## Transferring a Git project without SSH

Do this to create a zip archive of your project:

```bash
$ git archive -o ~/latest.zip HEAD
```

Then transfer it to another computer.

## Divergent branch reconciliation

If you get an error such as this:

```bash
$ git pull 
hint: You have divergent branches and need to specify how to reconcile them.
hint: You can do so by running one of the following commands sometime before
hint: your next pull:
hint: 
hint:   git config pull.rebase false  # merge (the default strategy)
hint:   git config pull.rebase true   # rebase
hint:   git config pull.ff only       # fast-forward only
hint: 
hint: You can replace "git config" with "git config --global" to set a default
hint: preference for all repositories. You can also pass --rebase, --no-rebase,
hint: or --ff-only on the command line to override the configured default per
hint: invocation.
fatal: Need to specify how to reconcile divergent branches.
```

then follow the instructions and choose one of the options. It's recommended to choose a global strategy, for example

```bash
$ git config pull.rebase --global true
```

## Automatic remote branch setup

If you get errors such as

```bash
$ git push
fatal: The current branch asdf has no upstream branch.
To push the current branch and set the remote as upstream, use

    git push --set-upstream origin asdf

```

and you usually resolve them by following the instructions:

```bash
$ git push --set-upstream origin asdf
```

you can set this to happen automatically with:

```bash
$ git config --global push.autoSetupRemote true
```

Note that according to [this page](https://medium.com/@anjusha.khandavalli/auto-setup-remote-branch-with-push-autosetupremote-892ddd7644), you need Git version `2.37.0` and later in order to do this. If your distro doesn't support that yet, follow the instructions [here](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) to install it from source:

```bash
$ sudo apt-get install dh-autoreconf libcurl4-gnutls-dev libexpat1-dev gettext libz-dev libssl-dev install-info
$ git clone --depth 1 --branch v2.43.0 https://github.com/git/git.git
$ make configure
$ make all
```

and then add the directory to your path.

If you're on Ubuntu, you can instead do

```bash
$ add-apt-repository ppa:git-core/ppa
$ apt update
$ apt install git
```

## Merge conflicts

Let's say you're doing a rebase. If you have two commits that both add a new Makefile rule, for example:

```
<<<<<<< HEAD
=======
docker:
	docker build . -t $(BUILD_IMAGE)
	docker push $(BUILD_IMAGE)

icon:
	yarn tauri icon src-tauri/icons/icon.png

>>>>>>> ee32ff6 (Add custom logo)
```

and you now want to rebase your latest commit so that these two changes are independent of each other, then you'll want to only keep the rule from the commit you care about.

If instead you're rebasing one commit onto another, then you'll likely want to keep both changes:

```
<<<<<<< HEAD
icon:
	yarn tauri icon src-tauri/icons/icon.png
=======
docker:
	docker build . -t $(BUILD_IMAGE)
	docker push $(BUILD_IMAGE)
>>>>>>> 8d08f05 (Dockerize build process)
```

You should use ZAMM to automatically resolve the merge conflict based on the semantic purposes of the changes.

## Submodules

If your repo state is dirty because of a submodule:

```bash
$ git status
On branch docker
Your branch is up to date with 'origin/docker'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   src-svelte/static/fonts (new commits)

no changes added to commit (use "git add" and/or "git commit -a")
```

which can happen simply by checking out a branch that points to a different commit of the submodule, you'll find that a `git reset --hard` does nothing because it only resets the files in the main repo, not the submodule. Instead, do this:

```bash
$ git submodule update --init
Submodule path 'src-svelte/static/fonts': checked out 'b5bf1655bcffe96126fb99c41e09b4d51e873fa3'
```

Now the repo state is clean:

```bash
$ git status
On branch docker
Your branch is up to date with 'origin/docker'.

nothing to commit, working tree clean
```

### Init from Git

If you're using a version of Git such as version 2.25.1, you may get a prompt like this when you try to clone a private submodule:

```bash
$ git submodule update --init
Cloning into '/home/amos/Documents/ui/zamm/src-svelte/static/fonts'...
Username for 'https://github.com': 
```

If you try to enter your username and password in, you'll only get more error:

```bash
Username for 'https://github.com': amosjyng
Password for 'https://amosjyng@github.com': 
remote: Support for password authentication was removed on August 13, 2021.
remote: Please see https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls for information on currently recommended modes of authentication.
fatal: Authentication failed for 'https://github.com/amosjyng/zamm-fonts.git/'
fatal: clone of 'https://github.com/amosjyng/zamm-fonts.git' into submodule path '/home/amos/Documents/ui/zamm/src-svelte/static/fonts' failed
Failed to clone 'src-svelte/static/fonts'. Retry scheduled

```

Follow [the link](https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls) to see that you'll want to get a PAT. Then repeat this again with the PAT.
