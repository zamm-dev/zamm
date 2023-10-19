# Ubuntu debugging

## Blank screen

Press ctrl-alt-<F3>. Login to the terminal. Then edit `/etc/gdm3/custom.conf` to set

```conf
[daemon]
AutomaticLoginEnable=false
```

If the system is crashing at the login screen too, you can also try setting

```conf
WaylandEnable=false
```

To start a different display manager such as IceWM, edit `~/.xinitrc`:

```
exec icewm-session
```

Then run

```bash
$ startx
```

## Flashing another distro to USB

If you use balenaEtcher and run into the problem `No polkit authentication agent found`, then follow the steps [here](https://askubuntu.com/a/1195823).
