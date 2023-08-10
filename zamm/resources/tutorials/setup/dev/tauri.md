# Setting up a Tauri dev environment

First, install Tauri dependencies:

```bash
$ sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

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
