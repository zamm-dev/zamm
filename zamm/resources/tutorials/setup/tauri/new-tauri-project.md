# Starting a new Tauri project

## Prerequisites

Set up NodeJS, Rust, and Tauri as described in:

- [`nodejs.md`](/zamm/resources/tutorials/setup/dev/nodejs.md)
- [`rust.md`](/zamm/resources/tutorials/setup/dev/rust.md)
- [`tauri.md`](/zamm/resources/tutorials/setup/dev/tauri.md)


Then set up Tauri app creation:

```bash
$ cargo install create-tauri-app --locked
    Updating crates.io index
  Downloaded create-tauri-app v3.7.2
  Downloaded 1 crate (262.6 KB) in 2.07s
...
   Compiling include-flate v0.2.0
   Compiling rust-embed v6.8.1
   Compiling create-tauri-app v3.7.2
    Finished release [optimized] target(s) in 27.98s
  Installing /home/amos/.asdf/installs/rust/1.71.0/bin/cargo-create-tauri-app
   Installed package `create-tauri-app v3.7.2` (executable `cargo-create-tauri-app`)
```

Finally, run `create-tauri-app`

```bash
$ cargo create-tauri-app
✔ Project name · zamm
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm)
✔ Choose your package manager · yarn
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
  cd zamm
  yarn
  yarn tauri dev
```

Follow the instructions:

```bash
$ cd zamm
$ yarn
yarn install v1.22.15
info No lockfile found.
[1/4] Resolving packages...

...

   Compiling webkit2gtk v0.18.2
    Finished dev [unoptimized + debuginfo] target(s) in 1m 41s
Could not determine the accessibility bus address
```

Note that the command does not exit.

Now edit `src-tauri/tauri.conf.json` to change the identifier from the default:

```json
{
      ...
      "active": true,
      "targets": "all",
      "identifier": "com.tauri.dev",
      "icon": [
        "icons/32x32.png",
        ...
      ]
}
```

to something else:

```json
{
      ...
      "active": true,
      "targets": "all",
      "identifier": "dev.zamm",
      "icon": [
        "icons/32x32.png",
        ...
      ]
}
```

Now run this in a new window:

```bash
$ cargo tauri build
...
    Finished 2 bundles at:
        /home/amos/projects/zamm/ui/zamm/src-tauri/target/release/bundle/deb/zamm_0.0.0_amd64.deb
        /home/amos/projects/zamm/ui/zamm/src-tauri/target/release/bundle/appimage/zamm_0.0.0_amd64.AppImage
```

Finally, you can run the generated AppImage. If it errors out with this message:

```bash
$ src-tauri/target/release/bundle/appimage/zamm_0.0.0_amd64.AppImage
dlopen(): error loading libfuse.so.2

AppImages require FUSE to run. 
You might still be able to extract the contents of this AppImage 
if you run it with the --appimage-extract option. 
See https://github.com/AppImage/AppImageKit/wiki/FUSE 
for more information
```

then install `fuse` and rerun the AppImage.

```bash
$ sudo apt install fuse
...
Processing triggers for man-db (2.10.2-1) ...
Processing triggers for libc-bin (2.35-0ubuntu3.1) ...
/sbin/ldconfig.real: /usr/lib/wsl/lib/libcuda.so.1 is not a symbolic link
```

You may want to enter this into the Tauri Makefile:

```Makefile
target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cargo build --release
  touch target/release/zamm  # cargo build might not do anything
```

Alternatively, you can just build the target without specifying dependencies, because Cargo is pretty performant anyways when everything is up to date.

Then in the main project Makefile:

```Makefile
build: svelte rust
	cargo tauri build

rust:
	cd src-tauri && make

svelte:
	cd src-svelte && make
```

where `src-tauri` and `src-svelte` are your respective directories for Svelte and Tauri code.

## Project dev tooling setup

Svelte setup:

Follow the instructions at:

- [`eslint.md`](/zamm/resources/tutorials/setup/tools/svelte/eslint.md)
- [`prettier.md`](/zamm/resources/tutorials/setup/tools/svelte/prettier.md)

Then for pre-commit, follow the instructions at

- [`pre-commit.md`](/zamm/resources/tutorials/setup/repo/pre-commit/pre-commit.md)
- [`cargo.md`](/zamm/resources/tutorials/setup/repo/pre-commit/cargo.md)
- [`svelte.md`](/zamm/resources/tutorials/setup/repo/pre-commit/svelte.md)

## Testing setup

Follow the instructions at

- [`e2e-testing.md`](./e2e-testing.md)
