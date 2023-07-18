# Setting up Python with `asdf`

`asdf` is a good way to manage multiple versions of programming language runtimes on your system. It also provides a unified interface to installing new plugins.

First, we set up `asdf` using the instructions [here](https://asdf-vm.com/guide/getting-started.html):

```bash
$ git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.12.0
Cloning into '/home/amos/.asdf'...
remote: Enumerating objects: 8459, done.
remote: Counting objects: 100% (372/372), done.
remote: Compressing objects: 100% (239/239), done.
remote: Total 8459 (delta 144), reused 310 (delta 127), pack-reused 8087
Receiving objects: 100% (8459/8459), 2.78 MiB | 475.00 KiB/s, done.
Resolving deltas: 100% (4988/4988), done.
Note: switching to '816195d615427b033a7426a4fb4d7fac4cf2d791'.

You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

If you want to create a new branch to retain commits you create, you may
do so (now or later) by using -c with the switch command. Example:

  git switch -c <new-branch-name>

Or undo this operation with:

  git switch -

Turn off this advice by setting config variable advice.detachedHead to false
```

Then, assuming that you're using the ZSH shell, add this to the end of `~/.zshrc`:

```
. "$HOME/.asdf/asdf.sh"
```

Then reload the conrfig file:

`. ~/.zshrc`

Add the plugin to asdf:

```bash
$ asdf plugin-add python
initializing plugin repository...Cloning into '/home/amos/.asdf/repository'...
remote: Enumerating objects: 5301, done.
remote: Counting objects: 100% (507/507), done.
remote: Compressing objects: 100% (89/89), done.
remote: Total 5301 (delta 434), reused 484 (delta 418), pack-reused 4794
Receiving objects: 100% (5301/5301), 1.21 MiB | 681.00 KiB/s, done.
Resolving deltas: 100% (2866/2866), done.
```

Then install the latest version of Python:

```bash
$ asdf install python latest
```

If you get an error such as this:

```bash
ModuleNotFoundError: No module named '_ssl'
ERROR: The Python ssl extension was not compiled. Missing the OpenSSL lib?

Please consult to the Wiki page to fix the problem.
https://github.com/pyenv/pyenv/wiki/Common-build-problems


BUILD FAILED (Ubuntu 22.04 using python-build 2.3.22-6-gb81204c0)
```

or warnings such as this:

```bash
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/home/amos/.asdf/installs/python/3.11.4/lib/python3.11/bz2.py", line 17, in <module>
    from _bz2 import BZ2Compressor, BZ2Decompressor
ModuleNotFoundError: No module named '_bz2'
WARNING: The Python bz2 extension was not compiled. Missing the bzip2 lib?
```

then install the OpenSSL and other libraries as such:

```bash
$ sudo apt-get install build-essential libssl-dev zlib1g-dev libffi-dev libbz2-dev libreadline-dev libsqlite3-dev liblzma-dev libncurses-dev tk-dev
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
...
```

and run the command again:

```bash
$ asdf install python latest
python-build 3.11.4 /home/amos/.asdf/installs/python/3.11.4
Downloading Python-3.11.4.tar.xz...
-> https://www.python.org/ftp/python/3.11.4/Python-3.11.4.tar.xz
Installing Python-3.11.4...
Installed Python-3.11.4 to /home/amos/.asdf/installs/python/3.11.4
```

There should be no errors or warnings now. Check that the `python` executable resolves to the asdf shim:

```bash
$ which python
/home/amos/.asdf/shims/python
```

Finally, set the version of Python that we've just downloaded as the global version:

```bash
$ asdf global python 3.11.4
```

With this last step, the output of `python --version` should look like this:

```bash
$ python --version
Python 3.11.4
```

rather than:

```bash
No version is set for command python
Consider adding one of the following versions in your config file at
python 3.11.4
```

## `poetry` setup

After setting up Python, we'll want to install poetry as well.

```bash
$ pip install poetry
Defaulting to user installation because normal site-packages is not writeable
Collecting poetry
  Downloading poetry-1.5.1-py3-none-any.whl (225 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 225.2/225.2 KB 1.1 MB/s eta 0:00:00
...
  WARNING: The script poetry is installed in '/home/amos/.local/bin' which is not on PATH.
  Consider adding this directory to PATH or, if you prefer to suppress this warning, use --no-warn-script-location.
Successfully installed attrs-23.1.0 build-0.10.0 cachecontrol-0.12.14 certifi-2023.5.7 charset-normalizer-3.2.0 cleo-2.0.1 crashtest-0.4.1 distlib-0.3.7 dulwich-0.21.5 filelock-3.12.2 html5lib-1.1 idna-3.4 importlib-metadata-6.8.0 installer-0.7.0 jaraco.classes-3.3.0 jsonschema-4.18.3 jsonschema-specifications-2023.6.1 keyring-23.13.1 lockfile-0.12.2 msgpack-1.0.5 pexpect-4.8.0 pkginfo-1.9.6 platformdirs-3.9.1 poetry-1.5.1 poetry-core-1.6.1 poetry-plugin-export-1.4.0 ptyprocess-0.7.0 pyproject-hooks-1.0.0 rapidfuzz-2.15.1 referencing-0.29.1 requests-2.31.0 requests-toolbelt-1.0.0 rpds-py-0.8.11 shellingham-1.5.0.post1 tomli-2.0.1 tomlkit-0.11.8 trove-classifiers-2023.7.6 urllib3-1.26.16 virtualenv-20.24.0 webencodings-0.5.1
```

Check which pip we're using. If the output is not under `.asdf`:

```bash
$ which pip
/usr/bin/pip
```

then you should start a new bash session:

```bash
$ which pip
/home/amos/.asdf/shims/pip
```

If it's still not right, then stop following these instructions until PATH is fixed.

If it is right, then continue on to fixing poetry:

```bash
$ pip install poetry
Collecting poetry
  Using cached poetry-1.5.1-py3-none-any.whl (225 kB)
...
Successfully installed SecretStorage-3.3.3 attrs-23.1.0 build-0.10.0 cachecontrol-0.12.14 certifi-2023.5.7 cffi-1.15.1 charset-normalizer-3.2.0 cleo-2.0.1 crashtest-0.4.1 cryptography-41.0.2 distlib-0.3.7 dulwich-0.21.5 filelock-3.12.2 html5lib-1.1 idna-3.4 importlib-metadata-6.8.0 installer-0.7.0 jaraco.classes-3.3.0 jeepney-0.8.0 jsonschema-4.18.3 jsonschema-specifications-2023.6.1 keyring-23.13.1 lockfile-0.12.2 more-itertools-9.1.0 msgpack-1.0.5 packaging-23.1 pexpect-4.8.0 pkginfo-1.9.6 platformdirs-3.9.1 poetry-1.5.1 poetry-core-1.6.1 poetry-plugin-export-1.4.0 ptyprocess-0.7.0 pycparser-2.21 pyproject-hooks-1.0.0 rapidfuzz-2.15.1 referencing-0.29.1 requests-2.31.0 requests-toolbelt-1.0.0 rpds-py-0.8.11 shellingham-1.5.0.post1 six-1.16.0 tomlkit-0.11.8 trove-classifiers-2023.7.6 urllib3-1.26.16 virtualenv-20.24.0 webencodings-0.5.1 zipp-3.16.2

[notice] A new release of pip is available: 23.1.2 -> 23.2
[notice] To update, run: pip3 install --upgrade pip
Reshimming asdf python...
```
