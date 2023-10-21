# Setting up Linux Mint

## Keyman

Follow some of the steps [here](https://help.keyman.com/knowledge-base/101) to create `~/.xsessionrc` as such:

```bash
export GTK_IM_MODULE=ibus
export XMODIFIERS=@im=ibus
export QT_IM_MODULE=ibus
```

Install some keyboard in Keyman. Start the ibus daemon, and then in ibus preferences, pick the language for the Keyman keyboard you just installed. It will then ask you to pick from different available keyboards for that language, and the one you just installed on Keyman should be visible.
