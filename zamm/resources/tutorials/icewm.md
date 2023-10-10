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
