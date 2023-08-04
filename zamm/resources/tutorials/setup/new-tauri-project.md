# Starting a new Tauri project

Install Tauri dependencies:
```bash
$ sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

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
[2/4] Fetching packages...
info fsevents@2.3.2: The platform "linux" is incompatible with this module.
...
success Saved lockfile.
warning Your current version of Yarn is out of date. The latest version is "1.22.19", while you're on "1.22.15".
info To upgrade, run the following command:
$ curl --compressed -o- -L https://yarnpkg.com/install.sh | bash
Done in 78.81s.
$ yarn tauri dev
yarn run v1.22.19
$ tauri dev
     Running BeforeDevCommand (`yarn dev`)
$ vite
Forced re-optimization of dependencies

...

   Compiling webkit2gtk v0.18.2
    Finished dev [unoptimized + debuginfo] target(s) in 1m 41s
Could not determine the accessibility bus address
```

Note that the command does not exit.

Now run this in a new terminal window:

```bash
$ cargo install tauri-cli
...
    Finished release [optimized] target(s) in 4m 25s
  Installing /home/amos/.asdf/installs/rust/1.71.0/bin/cargo-tauri
   Installed package `tauri-cli v1.4.0` (executable `cargo-tauri`)
```

Now, edit `src-tauri/tauri.conf.json` to change the identifier from the default:

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

Now run

```bash
$ tauri build
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
