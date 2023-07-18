# Setting up ZSH

ZSH is a popular alternative shell to bash. To set it up, first install it via `apt`:

```bash
$ sudo apt install zsh
[sudo] password for amos:
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following additional packages will be installed:
  zsh-common
Suggested packages:
  zsh-doc
The following NEW packages will be installed:
  zsh zsh-common
0 upgraded, 2 newly installed, 0 to remove and 56 not upgraded.
Need to get 4794 kB of archives.
After this operation, 18.2 MB of additional disk space will be used.
Do you want to continue? [Y/n] y
Get:1 http://archive.ubuntu.com/ubuntu jammy/main amd64 zsh-common all 5.8.1-1 [3985 kB]
Get:2 http://archive.ubuntu.com/ubuntu jammy/main amd64 zsh amd64 5.8.1-1 [809 kB]
Fetched 4794 kB in 4s (1283 kB/s)
Selecting previously unselected package zsh-common.
(Reading database ... 31340 files and directories currently installed.)
Preparing to unpack .../zsh-common_5.8.1-1_all.deb ...
Unpacking zsh-common (5.8.1-1) ...
Selecting previously unselected package zsh.
Preparing to unpack .../archives/zsh_5.8.1-1_amd64.deb ...
Unpacking zsh (5.8.1-1) ...
Setting up zsh-common (5.8.1-1) ...
Setting up zsh (5.8.1-1) ...
Processing triggers for man-db (2.10.2-1) ...
```

Then start up zsh:

```bash
$ zsh
This is the Z Shell configuration function for new users,
zsh-newuser-install.
You are seeing this message because you have no zsh startup files
(the files .zshenv, .zprofile, .zshrc, .zlogin in the directory
~).  This function can help you with a few settings that should
make your use of the shell easier.

You can:

(q)  Quit and do nothing.  The function will be run again next time.

(0)  Exit, creating the file ~/.zshrc containing just a comment.
     That will prevent this function being run again.

(1)  Continue to the main menu.

(2)  Populate your ~/.zshrc with the configuration recommended
     by the system administrator and exit (you will need to edit
     the file by hand, if so desired).

--- Type one of the keys in parentheses ---
```

Type in 2.

```bash
/home/amos/.zshrc:15: scalar parameter HISTFILE created globally in function zsh-newuser-install
```

Then install Oh My ZSH:

```bash
$ sh -c "$(wget https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh -O -)"
--2023-07-17 23:21:10--  https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh
Resolving raw.github.com (raw.github.com)... 185.199.111.133, 185.199.110.133, 185.199.108.133, ...
Connecting to raw.github.com (raw.github.com)|185.199.111.133|:443... connected.
HTTP request sent, awaiting response... 301 Moved Permanently
Location: https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh [following]
--2023-07-17 23:21:11--  https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh
Resolving raw.githubusercontent.com (raw.githubusercontent.com)... 185.199.110.133, 185.199.109.133, 185.199.108.133, ...
Connecting to raw.githubusercontent.com (raw.githubusercontent.com)|185.199.110.133|:443... connected.
HTTP request sent, awaiting response... 200 OK
Length: 18209 (18K) [text/plain]
Saving to: ‘STDOUT’

-                                  100%[================================================================>]  17.78K  --.-KB/s    in 0.008s

2023-07-17 23:21:11 (2.26 MB/s) - written to stdout [18209/18209]

Cloning Oh My Zsh...
remote: Enumerating objects: 1340, done.
remote: Counting objects: 100% (1340/1340), done.
remote: Compressing objects: 100% (1289/1289), done.
remote: Total 1340 (delta 30), reused 1140 (delta 27), pack-reused 0
Receiving objects: 100% (1340/1340), 1.99 MiB | 698.00 KiB/s, done.
Resolving deltas: 100% (30/30), done.
From https://github.com/ohmyzsh/ohmyzsh
 * [new branch]      master     -> origin/master
Branch 'master' set up to track remote branch 'master' from 'origin'.
Already on 'master'
/home/amos/projects

Looking for an existing zsh config...
Found /home/amos/.zshrc. Backing up to /home/amos/.zshrc.pre-oh-my-zsh
Using the Oh My Zsh template file and adding it to /home/amos/.zshrc.

Time to change your default shell to zsh:
Do you want to change your default shell to zsh? [Y/n] y
Changing your shell to /usr/bin/zsh...
[sudo] password for amos:
Shell successfully changed to '/usr/bin/zsh'.

         __                                     __
  ____  / /_     ____ ___  __  __   ____  _____/ /_
 / __ \/ __ \   / __ `__ \/ / / /  /_  / / ___/ __ \
/ /_/ / / / /  / / / / / / /_/ /    / /_(__  ) / / /
\____/_/ /_/  /_/ /_/ /_/\__, /    /___/____/_/ /_/
                        /____/                       ....is now installed!


Before you scream Oh My Zsh! look over the `.zshrc` file to select plugins, themes, and options.

• Follow us on Twitter: https://twitter.com/ohmyzsh
• Join our Discord community: https://discord.gg/ohmyzsh
• Get stickers, t-shirts, coffee mugs and more: https://shop.planetargon.com/collections/oh-my-zsh
```

Then, install the autocomplete plugin:

```bash
$ git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions
Cloning into '/home/amos/.oh-my-zsh/custom/plugins/zsh-autosuggestions'...
remote: Enumerating objects: 2460, done.
remote: Counting objects: 100% (75/75), done.
remote: Compressing objects: 100% (52/52), done.
remote: Total 2460 (delta 36), reused 49 (delta 21), pack-reused 2385
Receiving objects: 100% (2460/2460), 571.83 KiB | 227.00 KiB/s, done.
Resolving deltas: 100% (1567/1567), done.
```

And edit your `~/.zshrc`, which should have lines like this:

```
# Which plugins would you like to load?
# Standard plugins can be found in $ZSH/plugins/
# Custom plugins may be added to $ZSH_CUSTOM/plugins/
# Example format: plugins=(rails git textmate ruby lighthouse)
# Add wisely, as too many plugins slow down shell startup.
plugins=(git)
```

into this:

```
# Which plugins would you like to load?
# Standard plugins can be found in $ZSH/plugins/
# Custom plugins may be added to $ZSH_CUSTOM/plugins/
# Example format: plugins=(rails git textmate ruby lighthouse)
# Add wisely, as too many plugins slow down shell startup.
plugins=(git zsh-autosuggestions)
```

Source the config file after editing:

```bash
$ . ~/.zshrc
```

Now you should see ZSH with autocomplete as the default shell!
