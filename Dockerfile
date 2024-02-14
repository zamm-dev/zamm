FROM ubuntu:18.04
LABEL org.opencontainers.image.source="https://github.com/amosjyng/zamm"

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && \
  apt install -y --no-install-recommends build-essential libssl-dev zlib1g-dev libffi-dev libbz2-dev libreadline-dev libsqlite3-dev liblzma-dev libncurses-dev tk-dev libwebkit2gtk-4.0-dev curl wget file libgtk-3-dev librsvg2-dev ca-certificates software-properties-common patchelf && \
  apt-add-repository ppa:git-core/ppa && \
  apt update && \
  apt install -y git

ARG RUST_VERSION=1.71.1
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install tauri-cli

ARG NODEJS_VERSION=16.20.2
WORKDIR /tmp
RUN curl -SLO "https://nodejs.org/dist/v${NODEJS_VERSION}/node-v${NODEJS_VERSION}-linux-x64.tar.xz" && \
    tar -xJf "node-v${NODEJS_VERSION}-linux-x64.tar.xz" -C /usr/local --strip-components=1 && \
    npm install --global yarn pnpm json && \
    rm "node-v${NODEJS_VERSION}-linux-x64.tar.xz"

# project dependencies
RUN mkdir /tmp/dependencies
WORKDIR /tmp/dependencies
COPY package.json yarn.lock ./
COPY src-svelte/package.json ./src-svelte/package.json
COPY webdriver/package.json ./webdriver/package.json
RUN git clone https://github.com/amosjyng/neodrag.git src-svelte/forks/neodrag && \
  cd src-svelte/forks/neodrag && \
  git checkout e954f97 && \
  pnpm install && \
  pnpm compile && \
  cd /tmp/dependencies && \
  yarn

RUN apt install -y libasound2-dev

COPY src-tauri/Cargo.toml Cargo.toml
COPY src-tauri/Cargo.lock Cargo.lock
RUN git clone --depth 1 --branch zamm/v0.0.0 https://github.com/amosjyng/async-openai.git /tmp/forks/async-openai && \
  git clone --depth 1 --branch zamm/v0.0.0 https://github.com/amosjyng/rvcr.git /tmp/forks/rvcr && \
  mkdir src && \
  echo "// dummy file" > src/lib.rs && \
  echo "pub use tauri_build; fn main () {}" > build.rs && \
  cargo build --release --features custom-protocol
