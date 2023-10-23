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

and connect to `localhost:59000` with your local VNC client. On Ubuntu, this would be Vinagre. On KDE, this would be KRDC.

### Starting X server at startup

Create `/etc/systemd/system/vncserver@.service`:

```service
[Unit]
Description=Start TightVNC server at startup
After=syslog.target network.target

[Service]
Type=forking
User=root
Group=root
WorkingDirectory=/root

PIDFile=/root/.vnc/%H:%i.pid
ExecStartPre=-/usr/bin/vncserver -kill :%i > /dev/null 2>&1
ExecStart=/usr/bin/vncserver -depth 24 -geometry 1600x800 -localhost :%i
ExecStop=/usr/bin/vncserver -kill :%i

[Install]
WantedBy=multi-user.target
```

Then

```bash
$ sudo systemctl daemon-reload
$ sudo systemctl enable vncserver@1.service
Created symlink /etc/systemd/system/multi-user.target.wants/vncserver@1.service → /etc/systemd/system/vncserver@.service.
```

Start the service (kill the previous process if it's running):

```bash
$ sudo systemctl start vncserver@1
```

Check that it's running:

```bash
$ sudo systemctl status vncserver@1

● vncserver@1.service - Start TightVNC server at startup
     Loaded: loaded (/etc/systemd/system/vncserver@.service; enabled; vendor preset: enabled)
     Active: active (running) since Sun 2023-10-22 23:36:21 UTC; 6s ago
    Process: 60528 ExecStartPre=/usr/bin/vncserver -kill :1 > /dev/null 2>&1 (code=exited, status=0/SUCCESS)
    Process: 60535 ExecStart=/usr/bin/vncserver -depth 24 -geometry 1600x800 -localhost :1 (code=exited, status=0/SUCCESS)
   Main PID: 60546 (Xtightvnc)
      Tasks: 99 (limit: 9242)
     Memory: 255.8M
        CPU: 2.684s
     CGroup: /system.slice/system-vncserver.slice/vncserver@1.service
             ├─60546 Xtightvnc :1 -desktop X -auth /root/.Xauthority -geometry 1600x800 -depth 24 -rfbwait 120000 -rfbauth /root/.vnc/passwd -rfbport 5901 -fp /usr/share/fonts/X11/misc/,/usr/share>
             ├─60568 xfce4-session
```

You may want to add this to your shell init script, along with

```bash
export DISPLAY=:1.0
```

as described in [`tauri.md`](/zam/zamm/resources/tutorials/setup/dev/tauri.md).

### Errors

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
