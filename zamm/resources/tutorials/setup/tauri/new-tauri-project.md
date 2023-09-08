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

Finally, you can run the generated AppImage.

### Fuse Error

If it errors out with this message:

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

### Makefile edit

You may want to enter this into the Tauri Makefile:

```Makefile
target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cargo build --release
  touch target/release/zamm  # cargo build might not do anything
```

Alternatively, you can just build the target without specifying dependencies, because Cargo is pretty performant anyways when everything is up to date.

### GLIBC error

If on the other hand you see the problem

```bash
zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by zamm)
zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by zamm)
zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.34' not found (required by zamm)
...
```

then that means you should follow the instructions at [`cross.md`](zamm/resources/tutorials/setup/dev/cross.md) for compiling in a way that's compatible across Linux versions. We do this because static linking with musl is [not currently supported](https://github.com/tauri-apps/tauri/issues/5466) by Tauri, and compiling with an older version of glibc is necessary due to [the way GLIBC handles backwards-compatibility](https://developers.redhat.com/blog/2019/08/01/how-the-gnu-c-library-handles-backward-compatibility).

Afterwards, edit `src-tauri/tauri.conf.json` according to [its JSON schema](https://github.com/tauri-apps/tauri/blob/4cb51a2/tooling/cli/schema.json), because this feature appears to be undocumented anywhere except for [this issue](https://github.com/tauri-apps/tauri/issues/4255):

```json
{
  "build": {
    ...
    "runner": "cross"
  },
  ...
}
```

If you've edited `src-tauri/Makefile` as mentioned above, edit it again to use `cross` instead of `cargo`:

```Makefile
...

target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cross build --release --features custom-protocol
	touch target/release/zamm
```

Note that the build fails now because `src-svelte` is in another folder and therefore missing from the Docker container:

```
$ make
...
   Compiling tauri-specta v1.0.2
   Compiling zamm v0.0.0 (/project)
error: proc macro panicked
  --> src/main.rs:42:14
   |
42 |         .run(tauri::generate_context!())
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: message: The `distDir` configuration is set to `"../src-svelte/build"` but this path doesn't exist

warning: unused import: `std::env`
  --> src/main.rs:13:5
   |
13 | use std::env;
   |     ^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `zamm` (bin "zamm") generated 1 warning
error: could not compile `zamm` (bin "zamm") due to previous error; 1 warning emitted
make: *** [Makefile:4: target/release/zamm] Error 101
```

To fix this, we can mount additional volumes for cross as described [here](https://github.com/cross-rs/cross#mounting-volumes-into-the-build-environment). However, as described by [this issue](https://github.com/cross-rs/cross/issues/388), that doesn't actually work when we try it out and debug with `cross -vv ...`. Therefore, we use mkhattab's workaround in [this comment](https://github.com/cross-rs/cross/issues/388#issuecomment-1076862505) and edit our Makefile again:

```Makefile
...

target/release/zamm: ./Cargo.toml $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	DOCKER_OPTS="-v $(realpath ../src-svelte):/src-svelte" cross -vv build --release --features custom-protocol
	touch target/release/zamm

...
```

Now compilation works:

```bash
$ make
...
thiserror=/target/x86_64-unknown-linux-gnu/release/deps/libthiserror-3f9911903402c34c.rlib --extern uuid=/target/x86_64-unknown-linux-gnu/release/deps/libuuid-f91a98274275e90b.rlib --cfg desktop`
    Finished release [optimized] target(s) in 16.23s
touch target/release/zamm
```

Now we can remove the `-vv` option from cross debugging in the `src-tauri/Makefile`. However, we try running `make` in the overall directory, and note that it still fails on our local machine:

```bash
$ rsync -P -e ssh hetzner:/root/zamm/src-tauri/target/release/zamm ~/Downloads
zamm
     13,131,960 100%    2.30MB/s    0:00:05 (xfr#1, to-chk=0/1)
$ ./zamm
./zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.32' not found (required by ./zamm)
./zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.33' not found (required by ./zamm)
./zamm: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.34' not found (required by ./zamm)

```

Try running cross again inside `src-tauri` to confirm:

```bash
$ cross clean
$ make
```

Now it does work on the local development machine. This means that `cargo tauri build` is rebuilding the executable without cross.

### Continuing on...

Then in the main project Makefile:

```Makefile
build: svelte rust
	cargo tauri build

rust:
	cd src-tauri && make

svelte:
	cd src-svelte && make
```

where `src-tauri` and `src-svelte` are your respective directories for Svelte and Tauri code. If you want a global test command as well, add this:

```Makefile
test:
	cd src-python && make test
	cd src-svelte && make test
	cd src-tauri && make test
	yarn e2e-test
```

If you have followed the instructions for a [Svelte Makefile](zamm/resources/tutorials/setup/makefile/svelte.md), which would enable the above, you should get rid of the redundant `beforeBuildCommand` in `src-tauri/tauri.conf.json`.

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
