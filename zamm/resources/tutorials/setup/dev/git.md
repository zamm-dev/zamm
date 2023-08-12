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
