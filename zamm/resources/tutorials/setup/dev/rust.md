# Setting up Rust with `asdf`

Assuming that you already have `asdf` set up, next install the NodeJS plugin:

```bash
$ asdf plugin add rust
updating plugin repository...HEAD is now at b03baaa feat: add asdf-oapi-codegen plugin (#864)
```

Then install the latest version of Rust:

```bash
$ asdf install rust latest
info: downloading installer
info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
...
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

We ignore the Rust installer's instructions because asdf does that for us already. Instead, we mark the version we just installed as the global version of Rust:

```bash
$ asdf global rust 1.71.0
```

If `which rustc` is not showing a path under `~/.asdf/shims`, then close this shell session and start a new one:

```bash
$ which rustc
/home/amos/.asdf/shims/rustc
```

(Observed on GitHub CI) If you get error such as:

```
  --- stderr
  `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1" "pkg-config" "--libs" "--cflags" "gdk-3.0" "gdk-3.0 >= 3.22"` did not exit successfully: exit status: 1
  error: could not find system library 'gdk-3.0' required by the 'gdk-sys' crate

  --- stderr
  Package gdk-3.0 was not found in the pkg-config search path.
  Perhaps you should add the directory containing `gdk-3.0.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'gdk-3.0' found
  Package gdk-3.0 was not found in the pkg-config search path.
  Perhaps you should add the directory containing `gdk-3.0.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'gdk-3.0' found
```

## VS Code integration

If you are using VS Code with the rust-analyzer extension, then edit your user `settings.json` with an overridden path to asdf's rust install. For example, if you find that your `rust-analyzer` is now at this location:

```bash
$ which rust-analyzer
/home/amos/.asdf/shims/rust-analyzer
```

then link to it in the user's `settings.json` like so:

```json
{
  ...
  "rust-analyzer.server.path": "/home/amos/.asdf/shims/rust-analyzer",
  ...
}
```

If `rust-analyzer` doesn't yet exist, add that and `rust-src` as components to your Rust install:

```bash
$ rustup component add rust-src
$ rustup component add rust-analyzer
```

If you see an error such as

```bash
[ERROR rust_analyzer::main_loop] FetchWorkspaceError:
rust-analyzer failed to load workspace: Failed to load the project at /root/zamm/src-tauri/Cargo.toml: cd "/root/zamm/src-tauri" && "cargo" "--version" failed: No such file or directory (os error 2)
```

It is because you missed this step. If you are developing remotely, edit `.vscode/settings.json` to be:

```json
{
    "rust-analyzer.server.path": "/root/.asdf/shims/rust-analyzer"
}
```

and add this file to `.git/info/exclude`, from:

```
# git ls-files --others --exclude-from=.git/info/exclude
# Lines that start with '#' are comments.
# For a project mostly in C, the following would be a good set of
# exclude patterns (uncomment them if you want to use them):
# *.[oa]
# *~
```

to

```
# git ls-files --others --exclude-from=.git/info/exclude
# Lines that start with '#' are comments.
# For a project mostly in C, the following would be a good set of
# exclude patterns (uncomment them if you want to use them):
# *.[oa]
# *~
.vscode/settings.json
```

# Setting up Rust on Windows

Go [here](https://www.rust-lang.org/tools/install) and download the `rustup-init.exe` executable for your platform. Run it and follow the instructions. For example, a command prompt should pop up with a screen like this:

```
Rust Visual C++ prerequisites

Rust requires a linker and Windows API libraries but they don't seem to be
available.

These components can be acquired through a Visual Studio installer.

1) Quick install via the Visual Studio Community installer
   (free for individuals, academic uses, and open source).

2) Manually install the prerequisites
   (for enterprise and advanced users).

3) Don't install the prerequisites
   (if you're targeting the GNU ABI).

>
```

Press `1` for an easy install, and click through the various screens of the Visual Studio installer. Afterwards, you may see another screen such as

```
...
Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

Once again, press `1` to proceed with an easy installation.
