# Setting up a Tauri dev environment

First, install Tauri dependencies:

```bash
$ sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

Set up Rust using the instructions at [`rust.md`](/zamm/resources/tutorials/setup/dev/rust.md)

Then, install the Tauri CLI:

```bash
$ cargo install tauri-cli
...
    Finished release [optimized] target(s) in 4m 25s
  Installing /home/amos/.asdf/installs/rust/1.71.0/bin/cargo-tauri
   Installed package `tauri-cli v1.4.0` (executable `cargo-tauri`)
```

This could take a long time. If you're developing remotely, you may want to time this to compare cloud provider resources.

If you have a Tauri project already set up, head inside it and run `yarn` to install yarn dependencies as well. Otherwise, you may be faced with the error

```bash
$ yarn tauri dev                                             
yarn run v1.22.19                                                               
$ tauri dev                                                                     
/bin/sh: 1: tauri: not found                                                    
error Command failed with exit code 127.                                        
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.                              
```

## Remote dev

If instead you get the error

```bash
$ yarn tauri dev
yarn run v1.22.19
$ tauri dev
     Running BeforeDevCommand (`yarn dev`)
...
(zamm:91417): Gtk-WARNING **: 07:18:48.360: cannot open display: 
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

then you may need to follow [these instructions](/zamm/resources/tutorials/setup/dev/remote.md) to set up a VNC session, and then either run it from inside the VNC session, or see what the display is on the VNC session:

```bash
$ echo $DISPLAY
:1.0
```

then set the `DISPLAY` environment variable to that and try again:

```bash
$ export DISPLAY=:1.0
$ yarn tauri dev
```
