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

## Sound

Say you install `alsa-utils` and then run into this error when trying to run `alsamixer`:

```
$ alsamixer           
ALSA lib confmisc.c:855:(parse_card) cannot find card '0'
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_card_inum returned error: No such file or directory
ALSA lib confmisc.c:422:(snd_func_concat) error evaluating strings
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_concat returned error: No such file or directory
ALSA lib confmisc.c:1334:(snd_func_refer) error evaluating name
ALSA lib conf.c:5178:(_snd_config_evaluate) function snd_func_refer returned error: No such file or directory
ALSA lib conf.c:5701:(snd_config_expand) Evaluate error: No such file or directory
ALSA lib control.c:1528:(snd_ctl_open_noupdate) Invalid CTL default
cannot open mixer: No such file or directory
```

You'll probably want to create a dummy soundcard.

```bash
$ sudo modprobe snd-dummy

modprobe: FATAL: Module snd-dummy not found in directory /lib/modules/5.15.0-84-generic
```

It appears there is no `snd-dummy` for the kernel on Hetzner. We try `pulseaudio` instead:

```bash
$ pulseaudio --start  

W: [pulseaudio] main.c: This program is not intended to be run as root (unless --system is specified).
$ pulseaudio --start --system

E: [pulseaudio] main.c: --start not supported for system instances.
$ pulseaudio --system
```

Then in another terminal:

```bash
$ pactl load-module module-null-sink sink_name=Virtual_Sink
19
```

`alsamixer` will still fail, but at least ZAMM's sound-playing won't error out like it did before.
