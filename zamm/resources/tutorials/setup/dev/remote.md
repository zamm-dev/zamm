# Setting up a remote workstation

You'll probably want to upgrade it to the latest versions of various packages:

```bash
$ sudo apt update && apt upgrade
```

If a message comes up recommending that you restart, you should do so now

```bash
$ sudo reboot now
```

## VNC access

Follow the guide [here](https://www.digitalocean.com/community/tutorials/how-to-install-and-configure-vnc-on-ubuntu-20-04):

```bash
$ sudo apt install xfce4 xfce4-goodies tightvncserver
$ vncserver

You will require a password to access your desktops.

Password: 
Warning: password truncated to the length of 8.
Verify:   
Would you like to enter a view-only password (y/n)? n
xauth:  file /root/.Xauthority does not exist

New 'X' desktop is ubuntu-8gb-hil-1:1

Creating default startup script /root/.vnc/xstartup
Starting applications specified in /root/.vnc/xstartup
Log file is /root/.vnc/ubuntu-8gb-hil-1:1.log

$ vncserver -kill :1

Killing Xtightvnc process ID 88298
$ mv ~/.vnc/xstartup ~/.vnc/xstartup.bak
```

Edit `~/.vnc/xstartup` from

```
#!/bin/sh

xrdb "$HOME/.Xresources"
xsetroot -solid grey
#x-terminal-emulator -geometry 80x24+10+10 -ls -title "$VNCDESKTOP Desktop" &
#x-window-manager &
# Fix to make GNOME work
export XKL_XMODMAP_DISABLE=1
/etc/X11/Xsession
```

to

```
#!/bin/bash
xrdb $HOME/.Xresources
startxfce4 &
```

Start the VNC server again:

```bash
$ vncserver -localhost

New 'X' desktop is ubuntu-8gb-hil-1:1

Starting applications specified in /root/.vnc/xstartup
Log file is /root/.vnc/ubuntu-8gb-hil-1:1.log
```

You can add `-geometry 1600x800`

Now on your local machine:

```bash
ssh -L 59000:localhost:5901 -C -N -l root hetzner
```

and connect to `localhost:59000` with your local VNC client. On Ubuntu, this would be Vinagre.

If you get an error such as this when starting Firefox:

```bash
$ firefox
Client is not authorized to connect to ServerError: cannot open display: :1.0
```

then follow the advice of [this answer](https://askubuntu.com/a/1462654) and run

```bash
$ export XAUTHORITY=~/.Xauthority
```

and try starting Firefox again.
