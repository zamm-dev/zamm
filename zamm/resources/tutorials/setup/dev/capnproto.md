# Setting up Cap'n Proto for type-safety

Install the Python tool:

```bash
$ poetry add --group dev pycapnp
```

This is where we find out that stub generation is not officially supported, except through this [one repo](https://gitlab.com/mic_public/tools/python-helpers/capnp-stub-generator) that has not actually published a `capnp-stub-generator` package to PyPI as claimed.

```bash
$ pipx install capnp-stub-generator                
Fatal error from pip prevented installation. Full pip output in file:
    /root/.local/pipx/logs/cmd_2023-08-22_01.20.00_pip_errors.log

Some possibly relevant errors from pip install:
    ERROR: Could not find a version that satisfies the requirement capnp-stub-generator (from versions: none)
    ERROR: No matching distribution found for capnp-stub-generator

Error installing capnp-stub-generator.
```

Cap'n Proto is therefore useless for the specific purpose of ensuring consistent type-safety between Python and Rust.

## VS Code

The plugin `xmonader.vscode-capnp` is recommended.
