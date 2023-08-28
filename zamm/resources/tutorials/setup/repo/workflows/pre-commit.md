# Setting up a workflow for pre-commit

Follow the instructions [here](https://tobiasmcnulty.com/posts/caching-pre-commit/). Create the file `.github/workflows/pre-commit.yaml` with:

```yaml
name: pre-commit

on:
  pull_request:
  push:
    branches: [main]

jobs:
  pre-commit:
    runs-on: ubuntu-latest
    name: Pre-commit hooks
    steps:
      - uses: actions/checkout@v3
      - name: Install pre-commit
        run: |
          pipx install pre-commit==$PRE_COMMIT_VERSION
      - uses: actions/cache@v3
        with:
          path: ~/.cache/pre-commit/
          key: pre-commit-4|${{ env.PYTHON_VERSION }}
      - run: pre-commit run --show-diff-on-failure --color=always --all-files
```

If you have additional dependencies that depend on your local repo, then add them in prior to the `pre-commit run` step. For example, for NodeJS setup:

```yaml
      - name: Set up Yarn cache
        uses: actions/cache@v3
        continue-on-error: false
        with:
          path: |
            **/node_modules
            **/.eslintcache

          key: ${{ runner.os }}-yarn-${{ hashFiles('**/yarn.lock') }}
          restore-keys: |
            ${{ runner.os }}-yarn-
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: "20.5.1"
      - name: Install Node dependencies
        run: |
          yarn install
          cd src-svelte && yarn svelte-kit sync
```

If you want to extract the NodeJS version into a constant, make it an environment variable:

```yaml
env:
  NODEJS_VERSION: "20.5.1"
```

And then put the environment variable into the config:

```yaml
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: ${{ env.NODEJS_VERSION }}
```

For Python:

```yaml
      - name: Install poetry
        run: |
          pipx install poetry==$POETRY_VERSION
      - name: Setup Python
        uses: actions/setup-python@v4
        with:
          python-version: ${{ env.PYTHON_VERSION }}
          cache: poetry
      - name: Install Python dependencies
        run: poetry install
        working-directory: src-python
```

Note that you could `cd` into subdirectories as needed to install dependencies, or simply use `working-directory`.

For Rust:

```yaml
      - name: Set up cargo cache
        uses: Swatinem/rust-cache@v2
        continue-on-error: false
        with:
          workspaces: "src-tauri -> target"
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ env.RUST_VERSION }}
          override: true
```

To install dependencies with `apt-get` on Ubuntu:

```yaml
      - name: Install Tauri dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

## Chaining jobs

You can have jobs depend on each other. For example, you can refactor the above `jobs` entry into this:

```yaml
jobs:
  build:
    name: Build entire program
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Tauri dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      ...
      - name: Install Python dependencies
        run: poetry install
        working-directory: src-python
      # end of common dependency setup

      - name: Build artifacts
        run: make
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: builds
          path: |
            src-python/dist/main
            src-svelte/.svelte-kit/
            src-svelte/build/
            src-tauri/binaries/zamm-python-x86_64-unknown-linux-gnu
            src-tauri/target/release/zamm
            src-tauri/target/release/zamm-python
      - name: Upload final app
        uses: actions/upload-artifact@v3
        with:
          name: full-app
          path: |
            src-tauri/target/release/bundle/appimage/zamm_*.AppImage
            src-tauri/target/release/bundle/deb/zamm_*.deb
  pre-commit:
    name: Check pre-commit hooks
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v3
      - name: Install Tauri dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      ...
      - uses: actions/download-artifact@v3
        with:
          name: builds
      # end of common dependency setup with downloaded artifacts

      - name: Install pre-commit
        run: |
          pipx install pre-commit==$PRE_COMMIT_VERSION
      - uses: actions/cache@v3
        with:
          path: ~/.cache/pre-commit/
          key: pre-commit-4|${{ env.PYTHON_VERSION }}
      - run: pre-commit run --show-diff-on-failure --color=always --all-files
```

Note that you still have to set up the base tool dependencies, but you no longer have to install the dependencies for each language after downloading the artifacts uploaded in the previous stage.

To debug, go to your project's actions page. For example, if your username is `amosjyng` and your project is `zamm`, then the page will be `https://github.com/amosjyng/zamm-ui/actions`. Any uploaded artifacts can be downloaded from the "Summary" page.

Note that the uploads are separated into ones that are useful for each next stage, and ones that are useful to download but not actually used in the next job stage.

Finally, note that if these uploads are big, storage space for them may fill up quickly. You may want to either edit the retention period for the entire repo as described [here](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#configuring-the-retention-period-for-github-actions-artifacts-and-logs-in-your-repository) by going to a project settings page such as https://github.com/amosjyng/zamm/settings/actions, or by adding a custom artifact retention period as described [here](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts#configuring-a-custom-artifact-retention-period). For example:

```yaml
      - name: Upload final app
        uses: actions/upload-artifact@v3
        with:
          name: full-app
          path: |
            src-tauri/target/release/bundle/appimage/zamm_*.AppImage
            src-tauri/target/release/bundle/deb/zamm_*.deb
          retention-days: 1
```
