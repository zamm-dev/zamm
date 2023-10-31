# Setting up Linux Mint

## Keyman

Follow some of the steps [here](https://help.keyman.com/knowledge-base/101) to create `~/.xsessionrc` as such:

```bash
export GTK_IM_MODULE=ibus
export XMODIFIERS=@im=ibus
export QT_IM_MODULE=ibus
```

Install some keyboard in Keyman. Start the ibus daemon, and then in ibus preferences, pick the language for the Keyman keyboard you just installed. It will then ask you to pick from different available keyboards for that language, and the one you just installed on Keyman should be visible.

## Increasing swap size

Follow the instructions [here](https://old.reddit.com/r/linuxmint/comments/uhjyir/how_to_increase_swap_size/i76gsi9/):

```bash
$ sudo swapoff /swapfile
$ sudo fallocate -l 4G /swapfile
$ sudo mkswap /swapfile
mkswap: /swapfile: warning: wiping old swap signature.
Setting up swapspace version 1, size = 4 GiB (4294963200 bytes)
no label, UUID=1d12c995-fbfc-4947-b8a5-067a0a9e3721
$ sudo swapon /swapfile
```

To verify the change:

```bash
$ swapon --show
NAME      TYPE SIZE USED PRIO
/swapfile file   4G   2M   -2
```
