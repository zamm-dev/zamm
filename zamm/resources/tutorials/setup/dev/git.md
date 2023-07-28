# Setting up Git

Git probably exists but is not yet configured. To do so, ask the user for their name and email, and run the commands accordingly:

```bash
git config --global user.name "Amos Jun-yeung Ng"
git config --global user.email "me@amos.ng"
```

## Setting up SSH

To set up SSH for GitHub SSH access, we first generate SSH public-private keypairs:

```bash
$ ssh-keygen
```

Then, we add it to GitHub by going to [https://github.com/settings/keys](https://github.com/settings/keys) and clicking on "New SSH key". If you see a login page, then you should login first.
