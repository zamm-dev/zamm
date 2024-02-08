# Setting up SSH

## On the Mac

Go to `System Preferences > Sharing` and enable `Advanced > Remote Login`. This will start the SSH server on your Mac.

## Shortcut names

To set up shortcut names so that you don't have to type in the IP address every time, you can set up SSH shortcuts. To do this, open up `~/.ssh/config` and add the following:

```
Host hetzner
  HostName 123.123.123.123
  User root

```

Note that this doesn't change the name anywhere else, so if you want to use the shortcut name in other software (for example, VNC apps), you'll have to set it in `/etc/hosts` instead with a line that looks like:

```
123.123.123.123    hetzner
```

## Adding new SSH key to a server

You can SSH on to a server first with the password. Then, once you are in, you can see what your public key is by running this *locally*:

```bash
$ cat ~/.ssh/id_rsa.pub
```

Then, you can add this to the *server's* `~/.ssh/authorized_keys` file. If the file doesn't exist, you can create it.

Now the next time you SSH in, you won't need to use the password.
