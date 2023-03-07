Say you want to do the following task:

> Create a new Python project named hello-world


You note that any good project requires version control. We'll go with the most popular, `git`.

You proceed to use the terminal:

```bash
$ git init
Initialized empty Git repository in /tmp/zamm/scratch/hello-world/.git/
$ git checkout -b main
Switched to a new branch 'main'
$ wget https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore -O .gitignore
--2023-02-19 13:57:53--  https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore
Resolving raw.githubusercontent.com (raw.githubusercontent.com)... 185.199.108.133, 185.199.109.133, 185.199.110.133, ...
Connecting to raw.githubusercontent.com (raw.githubusercontent.com)|185.199.108.133|:443... connected.
HTTP request sent, awaiting response... 200 OK
Length: 3078 (3.0K) [text/plain]
Saving to: ‘.gitignore’

.gitignore          100%[===================>]   3.01K  --.-KB/s    in 0s      

2023-02-19 13:57:53 (81.4 MB/s) - ‘.gitignore’ saved [3078/3078]
```

You note that we need to set up licensing for this repo. We can start with the MIT license by default. However, the license includes a copyright message at the top bearing the copyright-holder's name. If you're an LLM, one trick you can do is to get the user's name from their Git config using the terminal:

You proceed to use the terminal:

```bash
$ git config --global user.name
Amos Jun-yeung Ng
```

You note that the name being output here. For me, it is "Amos Jun-yeung Ng" because that is my name. Whatever the output of this command is, put it into the `LICENSE` file when writing the MIT license.

You decide to edit the file `LICENSE`. It doesn't yet exist.

You write out to the file the contents

```
MIT License

Copyright (c) 2023 Amos Jun-yeung Ng

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

You proceed to use the terminal:

```bash
$ git add .
$ git commit -m "Initial commit with ZAMM"
[master (root-commit) 7a31990] Initial commit with ZAMM
 12 files changed, 751 insertions(+)
 create mode 100644 .flake8
 create mode 100644 .github/workflows/lint.yml
 create mode 100644 .github/workflows/test.yml
 create mode 100644 .gitignore
 create mode 100644 LICENSE
 create mode 100644 Makefile
 create mode 100644 README.md
 create mode 100644 hello_world/__init__.py
 create mode 100644 poetry.lock
 create mode 100644 pyproject.toml
 create mode 100644 tests/__init__.py
 create mode 100644 tests/test_pytest_works.py
```

You note that make sure no files are left behind

You proceed to use the terminal:

```bash
$ git status
On branch master
nothing to commit, working tree clean
```
That's all! **Don't take any more steps** because the task is now done!