# Setting up Jupyter notebook

Follow the instructions [here](https://jupyter.org/install). Run

```bash
$ pip install jupyterlab
$ jupyter lab
```

If you get an error such as

```
[C 2023-09-18 01:57:37.913 ServerApp] Running as root is not recommended. Use --allow-root to bypass.
```

and you are on a server where root is the default user, run it with the `--allow-root` option as recommended:

```bash
$ jupyter lab --allow-root
```
