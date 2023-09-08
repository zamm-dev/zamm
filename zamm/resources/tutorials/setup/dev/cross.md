# Setting up cross

First setup [Docker](./docker.md). Then follow the instructions [here](https://github.com/cross-rs/cross).

```bash
$ cargo install cross
```

If you used `asdf` to set up Rust, you will have to reshim:

```bash
$ asdf reshim rust
```

You can now try

```bash
$ cross build --target x86_64-unknown-linux-gnu --release
```

If you run into the error

```
  cargo:warning=`PKG_CONFIG_ALLOW_SYSTEM_CFLAGS="1" "pkg-config" "--libs" "--cflags" "glib-2.0" "glib-2.0 >= 2.48"` did not exit successfully: exit status: 1
  error: could not find system library 'glib-2.0' required by the 'glib-sys' crate

  --- stderr
  Package glib-2.0 was not found in the pkg-config search path.
  Perhaps you should add the directory containing `glib-2.0.pc'
  to the PKG_CONFIG_PATH environment variable
  No package 'glib-2.0' found
```

it is because some build dependencies [are missing](https://stackoverflow.com/questions/69609351/where-is-the-glib-2-0-file-on-linux). You can see [this discussion](https://github.com/cross-rs/cross/discussions/548) and try building your own docker image instead. Note that this error is happening from the Docker container, so installing the libraries on your local computer won't fix anything.

We take a look at which Docker image is being used:

```bash
$ docker images        
REPOSITORY                                  TAG       IMAGE ID       CREATED        SIZE
ghcr.io/cross-rs/x86_64-unknown-linux-gnu   main      396b40224753   13 days ago    1.46GB
hello-world                                 latest    9c7a54a9a43c   4 months ago   13.3kB
```

Create `src-tauri/Dockerfile`:

```Dockerfile
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu

RUN apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev librsvg2-dev
```

Note that we have left out `libayatana-appindicator3-dev` from the dependencies in [`tauri.md`](./tauri.md) because it cannot be found in this Docker image. You should find a way to install it yourself if you need it.

Now build it:

```bash
$ docker build src-tauri -t tauri-build
[+] Building 84.1s (6/6) FINISHED                                  docker:default
 => [internal] load build definition from Dockerfile                         0.0s
 => => transferring dockerfile: 197B                                         0.0s
 => [internal] load .dockerignore                                            0.0s
 => => transferring context: 2B                                              0.0s
 => [internal] load metadata for ghcr.io/cross-rs/x86_64-unknown-linux-gnu:  0.2s
 => CACHED [1/2] FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu@sha256:9e5b  0.0s
 => [2/2] RUN apt install -y libwebkit2gtk-4.0-dev build-essential curl wg  79.9s
 => exporting to image                                                       4.0s
 => => exporting layers                                                      4.0s
 => => writing image sha256:cdf40e2af660b98c438e61dc776a42cccf0eaf1cafc7c77  0.0s 
 => => naming to docker.io/library/tauri-build                               0.0s
```

Edit `src-tauri/Cargo.toml` to point to this image for our builds:

```toml
...

[package.metadata.cross.target.x86_64-unknown-linux-gnu]
image = "tauri-build"

...
```

Try again:

```bash
$ cross build --target x86_64-unknown-linux-gnu --release
   Compiling glib-sys v0.15.10
   Compiling gobject-sys v0.15.10
   Compiling gdk-sys v0.15.1
   Compiling gio-sys v0.15.10
error: failed to run custom build command for `glib-sys v0.15.10`

Caused by:
  process didn't exit successfully: `/target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build` (exit status: 1)
  --- stderr
  /target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.29' not found (required by /target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build)
  /target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.28' not found (required by /target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build)
  /target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build: /lib/x86_64-linux-gnu/libc.so.6: version `GLIBC_2.25' not found (required by /target/release/build/glib-sys-6df4cd6bc50fc7bf/build-script-build)
warning: build failed, waiting for other jobs to finish...
error: failed to run custom build command for `gdk-sys v0.15.1`
...
```

It appears we need to install a specific version of `libc`. Let's see what versions are available for install via `apt`:

```bash
$ docker run -d  --name throwaway tauri-build tail -f /dev/null
$ docker exec throwaway apt-cache policy libc6
libc6:
  Installed: 2.23-0ubuntu11.3
  Candidate: 2.23-0ubuntu11.3
  Version table:
 *** 2.23-0ubuntu11.3 500
        500 http://archive.archive.ubuntu.com/ubuntu xenial-updates/main amd64 Packages
        500 http://security.archive.ubuntu.com/ubuntu xenial-security/main amd64 Packages
        100 /var/lib/dpkg/status
     2.23-0ubuntu3 500
        500 http://archive.archive.ubuntu.com/ubuntu xenial/main amd64 Packages
```

It appears no other versions are available, so we may have to build it ourselves. Follow the instructions [here](https://iq.opengenus.org/install-specific-version-of-glibc/). We check out `https://ftp.gnu.org/gnu/libc/` and see what the lowest available version is that is still compatible with our build system. We see that they have `2.25` available in the HTML table:

```
[ ]	glibc-2.25.tar.bz2	2017-02-05 11:16 	20M	 
[ ]	glibc-2.25.tar.bz2.sig	2017-02-05 11:16 	455 	 
[ ]	glibc-2.25.tar.gz	2017-02-05 11:14 	26M	 
[ ]	glibc-2.25.tar.gz.sig	2017-02-05 11:14 	455 	 
[ ]	glibc-2.25.tar.xz	2017-02-05 11:17 	13M	 
[ ]	glibc-2.25.tar.xz.sig	2017-02-05 11:17 	455 	 
```

Let's download the smallest one, with a URL at `https://ftp.gnu.org/gnu/libc/glibc-2.25.tar.xz`:

```bash
$ apt install curl xz-utils
$ curl -O https://ftp.gnu.org/gnu/libc/glibc-2.25.tar.xz
$ tar -xf glibc-2.25.tar.xz
$ cd glibc-2.25
$ mkdir -p /opt/glibc-2.25-install
$ ./configure --prefix=/opt/glibc-2.25-install
checking build system type... x86_64-pc-linux-gnu
checking host system type... x86_64-pc-linux-gnu
checking for gcc... gcc
checking for suffix of object files... o
checking whether we are using the GNU C compiler... yes
checking whether gcc accepts -g... yes
checking for readelf... readelf
checking for g++... g++
checking whether we are using the GNU C++ compiler... yes
checking whether g++ accepts -g... yes
checking whether g++ can link programs... yes
configure: error: you must configure in a separate build directory
```

Very well, so that's why the tutorial tells us to create this seemingly superfluous directory. Follow the instructions:

```bash
$ mkdir build
$ cd build
$ ../configure --prefix=/opt/glibc-2.25-install
...
checking for nm... nm
checking for python3... no
checking for python... python
configure: error: 
*** These critical programs are missing or too old: gawk
*** Check the INSTALL file for required versions.
```

Update `gawk` and try again:

```bash
$ apt install gawk
$ ../configure --prefix=/opt/glibc-2.25-install
...
config.status: creating Makefile
config.status: creating config.h
config.status: executing default commands
$ make -j
$ make install
...
make[2]: Leaving directory '/glibc-2.25/elf'
make[1]: Leaving directory '/glibc-2.25'
```

It appears this was successful. Let's update our `src-tauri/Dockerfile` to use these new commands:

```Dockerfile
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu

RUN apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev librsvg2-dev curl xz-utils gawk && \
  curl -O https://ftp.gnu.org/gnu/libc/glibc-2.25.tar.xz && \
  tar -xf glibc-2.25.tar.xz && \
  mkdir glibc-2.25/build && \
  cd glibc-2.25/build && \
  mkdir -p /opt/glibc-2.25-install && \
  ../configure --prefix=/opt/glibc-2.25-install && \
  make -j && \
  make install

ENV LD_LIBRARY_PATH=/opt/glibc-2.25-install/lib:$LD_LIBRARY_PATH
ENV PATH=/opt/glibc-2.25-install/bin:$PATH

```

Try `cross` again:

```bash
$ cross build --target x86_64-unknown-linux-gnu --release
```

It returns immediately. Does it think everything is already up to date?

```bash
$ cross clean
[cross] note: Falling back to `cargo` on the host.
$ cross build --target x86_64-unknown-linux-gnu --release
```

Same thing. Only thing that exists is `src-tauri/target/CACHEDIR.TAG`. Try with more verbose logs:

```bash
$ cross -vv build --target x86_64-unknown-linux-gnu --r
elease
+ cargo metadata --format-version 1 --filter-platform x86_64-unknown-linux-gnu
+ rustc --print sysroot
+ rustup toolchain list
+ rustup target list --toolchain 1.71.1-x86_64-unknown-linux-gnu
+ rustup component list --toolchain 1.71.1-x86_64-unknown-linux-gnu
+ /usr/bin/docker
+ /usr/bin/docker run --userns host -e 'PKG_CONFIG_ALLOW_CROSS=1' -e 'XARGO_HOME=/xargo' -e 'CARGO_HOME=/cargo' -e 'CARGO_TARGET_DIR=/target' -e 'CROSS_RUNNER=' -e TERM -e BROWSER -e 'USER=root' --rm --user 0:0 -v /root/.xargo:/xargo:z -v /root/.asdf/installs/rust/1.71.1:/cargo:z -v /cargo/bin -v /root/zamm/src-tauri:/project:z -v /root/.asdf/installs/rust/1.71.1/toolchains/1.71.1-x86_64-unknown-linux-gnu:/rust:z,ro -v /root/zamm/src-tauri/target:/target:z -w /project -i -t tauri-build sh -c 'PATH=$PATH:/rust/bin cargo -vv build --target x86_64-unknown-linux-gnu --release'
+ rustup component list --toolchain 1.71.1-x86_64-unknown-linux-gnu
```

Let's try running it ourselves:

```bash
$ /usr/bin/docker run --userns host -e 'PKG_CONFIG_ALLOW_CROSS=1' -e 'XARGO_HOME=/xargo' -e 'CARGO_HOME=/cargo' -e 'CARGO_TARGET_DIR=/target' -e 'CROSS_RUNNER=' -e TERM -e BROWSER -e 'USER=root' --rm --user 0:0 -v /root/.xargo:/xargo:z -v /root/.asdf/installs/rust/1.71.1:/cargo:z -v /cargo/bin -v /root/zamm/src-tauri:/project:z -v /root/.asdf/installs/rust/1.71.1/toolchains/1.71.1-x86_64-unknown-linux-gnu:/rust:z,ro -v /root/zamm/src-tauri/target:/target:z -w /project -i -t -d --name throwaway tauri-build tail -f /dev/null
```

It exits right away. We try the old tried-and-true approach:

```bash
$ docker run -d --name throwaway tauri-build tail -f /dev/null
906e029c67f4154205d10e87c6ebc991a14e4c07b8f197371be71522845a1c67
$ docker exec -it throwaway /bin/bash
Error response from daemon: Container 906e029c67f4154205d10e87c6ebc991a14e4c07b8f197371be71522845a1c67 is not running
$ docker ps -a
CONTAINER ID   IMAGE         COMMAND               CREATED          STATUS                        PORTS     NAMES
906e029c67f4   tauri-build   "tail -f /dev/null"   12 seconds ago   Exited (139) 12 seconds ago             throwaway
$ docker logs throwaway
```

Incredibly, even `tail -f /dev/null` exits immediately on the container. There are no logs in the container. 

```bash
$ docker rm throwaway
throwaway
$ docker run -d --name throwaway tauri-build sleep infinity
d8e8a48abebf2e83392bdd4ce8bd377c0371303af6347532530d3a3b4c4bf813
$ docker ps -a
CONTAINER ID   IMAGE         COMMAND            CREATED          STATUS                        PORTS     NAMES
d8e8a48abebf   tauri-build   "sleep infinity"   45 minutes ago   Exited (139) 45 minutes ago             throwaway
```

Clearly, linking to the newly built GLIBC breaks all tools inside the image. Sure enough, if we rebuild while leaving the final `make install` step out, and manually export the environment variables, we get

```bash
# ldd
Segmentation fault (core dumped)
```

We would debug further, except that we have now discovered the [cross-toolchains repo](https://github.com/cross-rs/cross-toolchains), which appears to be made specifically for this. We follow the instructions:

```bash
$ cd ~/Documents
$ git clone https://github.com/cross-rs/cross
$ cd cross
$ git submodule update --init --remote
$ GLIBC_VERSION=2.25 cargo xtask configure-crosstool x86_64-unknown-linux-gnu
...
     Running `target/debug/xtask configure-crosstool x86_64-unknown-linux-gnu`
Error: 
   0: unable to find config for target "x86_64-unknown-linux-gnu"

Location:
   xtask/src/crosstool.rs:73

Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
Run with RUST_BACKTRACE=full to include source snippets.
```

It's unclear why this fails, because is on the list of [supported targets](https://github.com/cross-rs/cross#supported-targets). The issue [here](https://stackoverflow.com/a/76934730) also references that target. Nonetheless, to avoid this failure, we configure it for all targets:

```bash
$ GLIBC_VERSION=2.25 cargo xtask configure-crosstool
$ cargo build-docker-image x86_64-unknown-linux-gnu-sde-cross --tag local-base
...
 => => naming to ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:local-  0.0s
------
 > importing cache manifest from ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:main:
------
```

where `x86_64-unknown-linux-gnu-sde-cross` is pulled from the README table mapping for `x86_64-unknown-linux-gnu`.

We edit `src-tauri/Dockerfile` again:

```Dockerfile
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:local-base

RUN apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev librsvg2-dev 

```

and build it again:

```bash
$ cd /zamm/src-tauri
$ docker build . -t tauri-build
```

It gets stuck. We run it manually to see what's up:

```bash
$ docker run -d --name throwaway ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:local-base sleep infinity
$ docker exec -it throwaway /bin/bash
# apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev librsvg2-dev 
...
debconf: unable to initialize frontend: Dialog
debconf: (No usable dialog-like program is installed, so the dialog based frontend cannot be used. at /usr/share/perl5/Debconf/FrontEnd/Dialog.pm line 76.)
debconf: falling back to frontend: Readline
Configuring tzdata
------------------

Please select the geographic area in which you live. Subsequent configuration
questions will narrow this down by presenting a list of cities, representing the
time zones in which they are located.

  1. Africa        6. Asia            11. System V timezones
  2. America       7. Atlantic Ocean  12. US
  3. Antarctica    8. Europe          13. None of the above
  4. Australia     9. Indian Ocean
[More] 
```

We follow the instructions [here](https://serverfault.com/questions/1018355/can-not-stop-tzdata-asking-for-user-input-during-docker-compose-build) to edit the Dockerfile:

```Dockerfile
FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:local-base

RUN DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev librsvg2-dev 

```

Now it works:

```bash
$ docker build . -t tauri-build
...
 => [2/2] RUN DEBIAN_FRONTEND=noninteractive apt install -y --no-install-r  72.2s
 => exporting to image                                                       3.7s
 => => exporting layers                                                      3.7s
 => => writing image sha256:d9336be3fdd920a0aa29ae12e633dd28ef70536c5a27929  0.0s
 => => naming to docker.io/library/tauri-build                               0.0s
$ cross build --target x86_64-unknown-linux-gnu --release
...
   Compiling mockall v0.11.4
   Compiling diesel_migrations v2.1.0
    Finished release [optimized] target(s) in 3m 18s
```

Finally! But something is not right. We try running it one more time:

```bash
$ docker run -d --name throwaway ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:local-base sleep infinity
0ab765061c3e1df29f7d0ccd24f0a884ed7a6dbbdef31bf8afdea24f5a602e6e
$ docker exec -it throwaway /bin/bash
root@0ab765061c3e:/# ldd --version
ldd (Ubuntu GLIBC 2.31-0ubuntu9.9) 2.31
Copyright (C) 2020 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
Written by Roland McGrath and Ulrich Drepper.
```

Unfortunately, it's not using the version of GLIBC that we specified. Perhaps we shouldn't have ignored the "unable to find config for target" error earlier. Let's see what exactly the configuration command changed for us:

```bash
$ git status
On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
  (commit or discard the untracked or modified content in submodules)
        modified:   docker/cross-toolchains (new commits, modified content)
        modified:   docker/crosstool-config/arm-unknown-linux-gnueabihf.config

no changes added to commit (use "git add" and/or "git commit -a")
$ GLIBC_VERSION=2.25 cargo xtask configure-crosstool arm-unknown-linux-gnueabihf                                  
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/xtask configure-crosstool arm-unknown-linux-gnueabihf`
$ cd docker/cross-toolchains
$ git status
HEAD detached at 4299096
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   docker/crosstool-config/aarch64_be-unknown-linux-gnu-cross.config
        modified:   docker/crosstool-config/s390x-unknown-linux-gnu-cross.config

no changes added to commit (use "git add" and/or "git commit -a")
$ cd ../..
$ GLIBC_VERSION=2.25 cargo xtask configure-crosstool aarch64_be-unknown-linux-gnu-cross                        
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/xtask configure-crosstool aarch64_be-unknown-linux-gnu-cross`
$ ls docker/cross-toolchains/docker/crosstool-config 
aarch64_be-unknown-linux-gnu-cross.config  s390x-unknown-linux-gnu-cross.config
```

So it would appear that this is looking only for configs in `docker/crosstool-config` and `docker/cross-toolchains/docker/crosstool-config`. We ask about this problem [here](https://github.com/cross-rs/cross-toolchains/issues/41). In the meantime, we poke around and try to solve the problem ourselves by editing `docker/cross-toolchains/docker/Dockerfile.x86_64-unknown-linux-gnu-sde-cross`:

```Dockerfile
FROM ubuntu:18.04
...
```

We choose `18.04` because, looking at https://launchpad.net/ubuntu/+source/glibc, we see that as of September 8, 2023, the lowest glibc version greater than or equal to 2.25 on that page is 2.27 for Bionic Beaver, which is Ubuntu version 18.04.

```bash
$ docker run -d --name throwaway ghcr.io/cross-rs/x86_64-unknown-linux-gnu-sde-cross:local-base sleep infinity
36060a904a67f131dd18708a3c28f19c957c6449f071296d22e1afa4236dc097
$ docker exec -it throwaway ldd --version
ldd (Ubuntu GLIBC 2.27-3ubuntu1.6) 2.27
Copyright (C) 2018 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
Written by Roland McGrath and Ulrich Drepper.
```

Finally! But when we build our project, we get

```bash
$ make
...
  = note: /usr/bin/ld: cannot find -lsqlite3
          collect2: error: ld returned 1 exit status
          

error: could not compile `zamm` (bin "zamm") due to previous error
```

Add `libsqlite3-dev` to the Docker build.
