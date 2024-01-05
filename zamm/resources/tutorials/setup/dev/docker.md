# Installing Docker on Ubuntu

Follow [these instructions](https://docs.docker.com/engine/install/ubuntu/):

```bash
$ sudo apt-get update
$ sudo apt-get install ca-certificates curl gnupg
$ sudo install -m 0755 -d /etc/apt/keyrings
$ curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
$ sudo chmod a+r /etc/apt/keyrings/docker.gpg
$ echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
$ sudo apt-get update
$ sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

Check that it works:

```bash
$ sudo docker run hello-world
Unable to find image 'hello-world:latest' locally
latest: Pulling from library/hello-world
719385e32844: Pull complete 
Digest: sha256:dcba6daec718f547568c562956fa47e1b03673dd010fe6ee58ca806767031d1c
Status: Downloaded newer image for hello-world:latest

Hello from Docker!
This message shows that your installation appears to be working correctly.

To generate this message, Docker took the following steps:
...
```

## Tips and tricks

### Image debugging

If you want to open a shell to explore a Docker image, do

```bash
$ docker run -d --name throwaway tauri-build tail -f /dev/null
$ docker exec -it throwaway /bin/bash
```

You can skip the first step if you already have the container built.

If you changed your Dockerfile and want to destroy and recreate a container based on the newer image, you can do

```bash
$ docker stop throwaway                
throwaway
$ docker rm throwaway
throwaway
```

If you see an error such as

```
Error response from daemon: Cannot kill container: throwaway: Container 9e1f033ae66fc64fa335332033b706fa77f8acf41a7e7937e9740338914f69ea is not running
```

that means the container is already stopped, and you can proceed directly to `docker rm`.

### Logging

To see the logs of a container you just started, do

```bash
$ docker logs -f throwaway
```

### Clean and reset

To completely nuke all existing Docker images, containers, and volumes, do

```bash
$ docker system prune -a
```

### Python installation

A typical Python install inside a Docker container might look like this:

```Dockerfile
ARG PYTHON_VERSION=3.11.4
WORKDIR /tmp
RUN wget https://www.python.org/ftp/python/${PYTHON_VERSION}/Python-${PYTHON_VERSION}.tgz && \
  tar -xvf Python-${PYTHON_VERSION}.tgz && \
  cd Python-${PYTHON_VERSION} && \
  ./configure --enable-shared && \
  make -j && \
  make install && \
  ldconfig && \
  pip3 install poetry && \
  rm -rf /tmp/Python-${PYTHON_VERSION}*
```

#### Errors

If you get an error such as

```
0.196 --2023-09-08 06:34:25--  https://www.python.org/ftp/python/3.11.4/Python-3.11.4.tgz
0.206 Resolving www.python.org (www.python.org)... 199.232.144.223, 2a04:4e42:64::223
0.212 Connecting to www.python.org (www.python.org)|199.232.144.223|:443... connected.
0.218 ERROR: cannot verify www.python.org's certificate, issued by 'CN=GlobalSign Atlas R3 DV TLS CA 2023 Q2,O=GlobalSign nv-sa,C=BE':
0.218   Unable to locally verify the issuer's authority.
0.218 To connect to www.python.org insecurely, use `--no-check-certificate'.
```

try installing `ca-certificates` as mentioned [here](https://unix.stackexchange.com/a/445609).

### Rust installation

A typical Rust installation inside a Docker container might look like this:

```Dockerfile
ARG RUST_VERSION=1.71.1
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install tauri-cli
```

#### Caching dependencies

Because `cargo` does not support only downloading dependencies for caching, you'll have to do a workaround as described in [this answer](https://stackoverflow.com/a/49664709):

```Dockerfile
RUN mkdir /tmp/dependencies
WORKDIR /tmp/dependencies
COPY src-tauri/Cargo.toml Cargo.toml
COPY src-tauri/Cargo.lock Cargo.lock
RUN mkdir src/ && \
  echo "// dummy file" > src/lib.rs && \
  cargo build --release
```

Afterwards, during the actual build phase:

```bash
mv /tmp/dependencies/target ./src-tauri/target 
```

Caching dependencies may make less sense for languages where dependency resolution and download are really fast, such as JS, because your base image will be out of date every time you add a new dependency. However, for a language such as Rust, where compilation of dependencies may take a long time, it may make sense to do so.

You'll want to check your Dockerized build logs. If they show that dependencies are still being built:

```
cargo build --release --features custom-protocol
   Compiling embed-resource v2.2.0
   Compiling tauri v1.4.1
   Compiling tauri-winres v0.1.1
   Compiling tauri-macros v1.4.0
   Compiling cargo_toml v0.15.3
   Compiling tauri-build v1.4.0
   Compiling specta v1.0.5
   Compiling zamm v0.0.0 (/__w/zamm-ui/zamm-ui/src-tauri)
   Compiling tauri-specta v1.0.2
    Finished release [optimized] target(s) in 1m 01s
```

it may be because you haven't specified the same exact features you're going to use at build time. After syncing the two commands, we see that there are still some libraries Cargo hasn't compiled:

```
cargo build --release --features custom-protocol
   Compiling embed-resource v2.2.0
   Compiling cargo_toml v0.15.3
   Compiling tauri-winres v0.1.1
   Compiling tauri-build v1.4.0
   Compiling zamm v0.0.0 (/__w/zamm-ui/zamm-ui/src-tauri)
    Finished release [optimized] target(s) in 46.21s
```

This is because of the `tauri-build` build dependency. We add one more line:

```Dockerfile
RUN mkdir src/ && \
  ...
  echo "pub use tauri_build; fn main () {}" > build.rs && \
  ...
```

We can use [this trick](https://stackoverflow.com/a/65582435) to see our Docker build output to verify that the remaining libraries are being compiled in the image.

### NodeJS installation

A typical NodeJS installation inside a Docker container might look like this:

```Dockerfile
ARG NODEJS_VERSION=16.20.2
WORKDIR /tmp
RUN curl -SLO "https://nodejs.org/dist/v${NODEJS_VERSION}/node-v${NODEJS_VERSION}-linux-x64.tar.xz" && \
    tar -xJf "node-v${NODEJS_VERSION}-linux-x64.tar.xz" -C /usr/local --strip-components=1 && \
    npm install --global yarn pnpm json && \
    rm "node-v${NODEJS_VERSION}-linux-x64.tar.xz"
```

#### Caching dependencies

If you want to cache your NodeJS dependencies so that the build can run faster next time, you can do

```Dockerfile
RUN mkdir /tmp/dependencies
WORKDIR /tmp/dependencies
COPY package.json yarn.lock ./
COPY src-svelte/package.json ./src-svelte/package.json
COPY webdriver/package.json ./webdriver/package.json
RUN yarn
```

Then during your actual build with real project files, you can do something like

```bash
mv /tmp/dependencies/node_modules ./node_modules
mv /tmp/dependencies/src-svelte/node_modules ./src-svelte/node_modules
```

Note that if you are using `pnpm`, you may need to add `.pnpm-store` to your `.gitignore` because it appears the default caching location of `~/.pnpm-store` could resolve to just the current directory on Docker.

If you need to get rid of some dependencies because you've forked them and don't want to copy their whole build into the local repo, you can do

```Dockerfile
RUN json -I -f src-svelte/package.json \
         -e 'delete this.dependencies["@neodrag/svelte"]'
```

to remove one specific dependency. See the `json` tool [documentation](https://trentm.com/json/#FEATURE-In-place-editing) for more information. You can also use other alternatives described [here](https://stackoverflow.com/questions/43292243/how-to-modify-a-keys-value-in-a-json-file-from-command-line).

##### Copying cached dependencies

After building the Docker image, you can copy the cached dependencies like so:

```Makefile
copy-docker-deps:
	mv -n /tmp/dependencies/src-svelte/forks/neodrag/packages/svelte/dist ./src-svelte/forks/neodrag/packages/svelte/dist
	mv -n /tmp/dependencies/node_modules ./node_modules
	mv -n /tmp/dependencies/src-svelte/node_modules ./src-svelte/node_modules
	mv -n /tmp/dependencies/target ./src-tauri/target

build-docker:
	docker run --rm -v $(CURRENT_DIR):/zamm -w /zamm $(BUILD_IMAGE) make copy-docker-deps build
```

You may want the `-n` option as shown [here](https://unix.stackexchange.com/a/248547) so that even if the build is not clean, the `build-docker` step will continue instead of erroring out and forcing you to clean, which would cause everything to need to be rebuilt again.

## Pushing to remote registry

### GitHub

Create a PAT as mentioned [here](/zamm/resources/tutorials/coding/frameworks/sveltekit.md) that has permissions to read, write, and delete packages. Put the key in an environmental variable such as `GHCR_PAT`. Now suppose your GitHub username is `amosjyng` and you have a local image `zamm-build` that you want to push to the repo at `github.com/amosjyng/zamm`. Then do

```bash
$ docker login ghcr.io -u amosjyng -p $GHCR_PAT
WARNING! Using --password via the CLI is insecure. Use --password-stdin.
WARNING! Your password will be stored unencrypted in /root/.docker/config.json.
Configure a credential helper to remove this warning. See
https://docs.docker.com/engine/reference/commandline/login/#credentials-store

Login Succeeded
$ docker tag zamm-build ghcr.io/amosjyng/zamm:v0.0.0-build
$ docker push ghcr.io/amosjyng/zamm:v0.0.0-build
```

You'll want to now visit https://github.com/users/amosjyng/packages/container/zamm/settings and set its visibility to public if it's an open-source project, and allow your repos to access it in GitHub actions.

#### Makefile

If you are using a Makefile for your build, you can set it as such:

```
BUILD_IMAGE = ghcr.io/amosjyng/zamm:v0.0.0-build
CURRENT_DIR = $(shell pwd)

build-docker:
	docker run --rm -v $(CURRENT_DIR):/zamm -w /zamm $(BUILD_IMAGE) make build

build: python svelte rust
	cargo tauri build

docker:
	docker build . -t $(BUILD_IMAGE)
	docker push $(BUILD_IMAGE)
```

where `build-docker` uses Docker to run the `build` command.

##### Cleaning

To clean up artifacts to make sure that you are actually successfully building everything from scratch, you can do

```Makefile
clean:
	cd src-python && make clean
	cd src-svelte && make clean
	cd src-tauri && make clean
```

Then clean up whatever files are generated in each subdirectory. For example, in `src-python/Makefile`:

```Makefile
...

clean:
	rm -rf build dist
```

Meanwhile, in `src-svelte/Makefile`:

```Makefile
...

clean:
	rm -rf build
```

## CI

To use this in GitHub CI, follow the instructions in the previous section for pushing to GHCR, the GitHub Container Registry. Then, look at [this answer](https://stackoverflow.com/a/74217028) for how to parameterize the container image used in the workflow.

If you get an error such as:

```
Invalid workflow file: .github/workflows/tests.yaml#L1
No steps defined in `steps` and no workflow called in `uses` for the following jobs: prepare-image
```

that's because you didn't specify the dummy step mentioned in the code above.

If you get an error such as

```
Error: Input 'submodules' not supported when falling back to download using the GitHub REST API. To create a local Git repository instead, add Git 2.18 or higher to the PATH.
```

it is because you need to install a recent version of Git into your Docker image. See [this issue](https://github.com/actions/checkout/issues/758) and [this answer](https://askubuntu.com/a/568596) for how to install the latest version of Git on Ubuntu, because the default Git for older versions of Ubuntu may only be 2.17. To check the Git version before running tests:

```bash
$ docker run --rm -v $(pwd):/zamm -w /zamm ghcr.io/amosjyng/zamm:v0.0.0-build git --version
git version 2.42.0
```

If you get an error such as

```
/__w/_temp/5681534b-2316-4cc1-b769-4adaa19dd285.sh: 1: /__w/_temp/5681534b-2316-4cc1-b769-4adaa19dd285.sh: pre-commit: not found
Error: Process completed with exit code 127.
```

it is because somehow `pipx`'s installation of `pre-commit` did not make it into the PATH. Either add it to the environment variables in the workflow as suggested [here](https://stackoverflow.com/a/68214331):

```yaml
jobs:
	...
  pre-commit:
    ...
    steps:
      ...
      - name: Add pre-commit to PATH
        run: echo "/root/.local/pipx/venvs/pre-commit/bin" >> $GITHUB_PATH
			- run: pre-commit run --show-diff-on-failure --color=always --all-files
```

or put it into your Dockerfile as suggested [here](https://stackoverflow.com/a/68758943):

```Dockerfile
...
ENV PATH="${PATH}:/root/.local/pipx/venvs/pre-commit/bin"

...

COPY .pre-commit-config.yaml .
RUN git init . && \
  pre-commit install-hooks
```

Now you might get an error such as

```
An error has occurred: FatalError: git failed. Is it installed, and are you in a Git repository directory?
Check the log at /github/home/.cache/pre-commit/pre-commit.log
```

We look at the CI steps:

```
  /usr/bin/git init /__w/zamm-ui/zamm-ui
```

The repo is initialized there. We run `pwd` in the job right before the pre-commit step. We see that we are in the right folder after all:

```
WARNING: Error loading config file: /root/.docker/config.json: open /root/.docker/config.json: permission denied
/__w/zamm-ui/zamm-ui
```

Let's try exporting the pre-commit log after all.

While researching this, [we find](https://github.com/actions/checkout/issues/841#issuecomment-1220502440) that we should be running this as the root user. We try again:

```yaml
  pre-commit:
    name: Check pre-commit hooks
    runs-on: ubuntu-latest
    needs: prepare-image
    container:
      image: ${{ needs.prepare-image.outputs.image }}
      options: --user root
    steps:
      - uses: actions/checkout@v3
        with:
          token: ${{ secrets.FONTS_PAT }}
          submodules: "recursive"
      - run: pwd
```

If you get an error such as

```
cargo build --release --features custom-protocol
error: rustup could not choose a version of cargo to run, because one wasn't specified explicitly, and no default is configured.
help: run 'rustup default stable' to download the latest stable release of Rust and set it as your default toolchain.
make[1]: *** [target/release/zamm] Error 1
Makefile:4: recipe for target 'target/release/zamm' failed
make[1]: Leaving directory '/__w/zamm-ui/zamm-ui/src-tauri'
make: *** [rust] Error 2
Makefile:42: recipe for target 'rust' failed
Error: Process completed with exit code 2.
```

see what toolschains made it into the image:

```
$ docker run --rm ghcr.io/amosjyng/zamm:v0.0.0-build rustup tool
chain list
1.71.1-x86_64-unknown-linux-gnu (default)
```

Then try to add that to your `.github/workflows/tests.yaml` to debug:

```yaml
jobs:
  ...
  build:
    ...
    steps:
      ...
      - run: rustup toolchain list
```

The output of this step on the CI server is:

```
no installed toolchains
```

Let's check the image download logs on the CI build:

```
  ...
  ef8fdfc391be: Pull complete
  Digest: sha256:72b56197483c9fed714a773b4b1239708f43c1a15ecb53328b3d6836de74b629
  Status: Downloaded newer image for ghcr.io/amosjyng/zamm:v0.0.0-build
	...
```

Using [this answer](https://stackoverflow.com/a/33511811), we check the SHA of the Docker image:

```bash
$ docker inspect --format='{{index .RepoDigests 0}}' ghcr.io/amosjyng/zamm:v0.0.0-build
ghcr.io/amosjyng/zamm@sha256:72b56197483c9fed714a773b4b1239708f43c1a15ecb53328b3d6836de74b629
```

It checks out. Let's see about the toolchains:

```bash
$ docker run --rm -v $(pwd):/zamm -w /zamm ghcr.io/amosjyng/zamm:v0.0.0-build ls -la ~/.rustup/toolchains
total 12
drwxr-xr-x 3 root root 4096 Sep 11 07:43 .
drwxr-xr-x 6 root root 4096 Sep 11 07:43 ..
drwxr-xr-x 7 root root 4096 Sep 11 07:43 1.71.1-x86_64-unknown-linux-gnu
$ docker run --rm -v $(pwd):/zamm -w /zamm ghcr.io/amosjyng/zamm:v0.0.0-build whoami                         
root
```

We observe the GitHub CI container logs:

```
	...
  /usr/bin/docker inspect --format "{{range .Config.Env}}{{println .}}{{end}}" 270de6d2a6e6916e823d7b444bdae3186c0a2b29b0684e4e1dc6b820dffd9699
  HOME=/github/home
	...
```

HOME should point to `/root` instead of `/github/home`. The picture is coming together. With a configuration such as this:

```yaml
  build:
    name: Build entire program
    runs-on: ubuntu-latest
    needs: prepare-image
    container:
      image: ${{ needs.prepare-image.outputs.image }}
      options: --user root
    env:
      HOME: /root
    steps:
      - uses: actions/checkout@v3
        with:
          token: ${{ secrets.FONTS_PAT }}
          submodules: "recursive"
      - name: Build artifacts
        run: make build
			...
```

the build finally works. However, this is still the only step that works. Seeing as using the Docker build does not actually save us any time, and even takes more time for some steps such as Python and the Webdriver end-to-end tests, we give up on this endeavor for now.

### Errors

If you get the message

```
Waiting for a runner to pick up this job
```

and the job never starts, it may be because you specified the run [incorrectly](https://stackoverflow.com/a/70968478). It should be

```yaml
runs-on: ubuntu-latest
container:
  image: YOUR_IMAGE
```

instead of

```yaml
runs-on: YOUR_IMAGE
```
