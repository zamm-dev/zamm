# IceWM

## Setting brightness

Taking a page from [this gist](https://gist.github.com/cho2/b714350eb0ea28b1d88d375f5d6ce7df):

```bash
$ ls /sys/class/backlight/
amdgpu_bl0
$ cat /sys/class/backlight/amdgpu_bl0/brightness
15
$ echo 10 | sudo tee /sys/class/backlight/amdgpu_bl0/brightness
10
```

## Setting audio volume

Run

```bash
$ alsamixer
```

Press F6 if necessary to pick the sound card, and then hit the up and down arrow keys to raise or lower the master volume control.

## Setting a 24 hour clock

As documented [here](https://ice-wm.org/man/icewm-preferences.html):

> TimeFormat=”%X”
>
> The clock time format. See the strftime manpage for the meaning of all the percent options. It is possible to define multiple clocks for different time zones in a single TimeFormat. A new clock is defined by the beginning of the string, and by each time zone specification that starts with TZ=..., followed by a space. For example, TimeFormat=%X TZ=Asia/Aden %T TZ=Asia/Baku %T defines 3 clocks.

So edit `~/.icewm/preferences` to contain:

```ini
TimeFormat=%H:%M:%S
```

and

```bash
$ icewm -r
```

to reload the IceWM configuration.

## Natural scrolling

Follow [these instructions](https://askubuntu.com/questions/1122513/how-to-add-natural-inverted-mouse-scrolling-in-i3-window-manager) and edit `/usr/share/X11/xorg.conf.d/40-libinput.conf` to add `Option "NaturalScrolling" "True"` inside of here:

```ini
Section "InputClass"
        Identifier "libinput pointer catchall"
        MatchIsPointer "on"
        MatchDevicePath "/dev/input/event*"
        Driver "libinput"
        Option "NaturalScrolling" "True"
EndSection
```
