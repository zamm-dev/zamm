# Setting up Rust with `asdf`

Assuming that you already have `asdf` set up, next install the NodeJS plugin:

```bash
$ asdf plugin-add rust
updating plugin repository...HEAD is now at b03baaa feat: add asdf-oapi-codegen plugin (#864)
```

Then install the latest version of Rust:

```bash
$ asdf install rust latest
info: downloading installer
info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for '1.71.0-x86_64-unknown-linux-gnu'
info: latest update on 2023-07-13, rust version 1.71.0 (8ede3aae2 2023-07-12)
info: downloading component 'cargo'
  7.0 MiB /   7.0 MiB (100 %) 1023.3 KiB/s in 17s ETA:  0s
info: downloading component 'clippy'
  2.3 MiB /   2.3 MiB (100 %)   1.0 MiB/s in  5s ETA:  0s
info: downloading component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %) 679.4 KiB/s in 24s ETA:  0s 
info: downloading component 'rust-std'
 25.4 MiB /  25.4 MiB (100 %)   1.0 MiB/s in 50s ETA:  0s
info: downloading component 'rustc'
 64.0 MiB /  64.0 MiB (100 %)   1.0 MiB/s in  1m 47s ETA:  0s 
info: downloading component 'rustfmt'
  2.3 MiB /   2.3 MiB (100 %)   1.0 MiB/s in  4s ETA:  0s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %)   9.9 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 25.4 MiB /  25.4 MiB (100 %)  16.8 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 64.0 MiB /  64.0 MiB (100 %)  18.1 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to '1.71.0-x86_64-unknown-linux-gnu'

  1.71.0-x86_64-unknown-linux-gnu installed - rustc 1.71.0 (8ede3aae2 2023-07-12)


Rust is installed now. Great!

To get started you need Cargo's bin directory 
(/home/amos/.asdf/installs/rust/1.71.0/bin) in your PATH
environment variable. This has not been done automatically.

To configure your current shell, run:
source "/home/amos/.asdf/installs/rust/1.71.0/env"
```

We ignore the Rust installer's instructions because asdf does that for us already. Instead, we mark the version we just installed as the global version of NodeJS:

```bash
$ asdf global rust 1.71.0
```
