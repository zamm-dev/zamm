# Initializing a Git repo

Every non-trivial project requires version control. The most popular version control system, `git`, ,is a sensible default to use for this purpose. So, if you are given a task such as:

> Initialize a Git repo

then you can do so by following these steps:

The very first command we run is to initialize the repo in the current directory.

```bash
$ git init
Initialized empty Git repository in /tmp/zamm/scratch/hello-world/.git/
```

Then, we change the primary branch to `main` because it requires fewer keystrokes than `master`.

```bash
$ git checkout -b main
Switched to a new branch 'main'
```

Next, because we're creating a Python project, we download the Python `.gitignore` file from Github:

```bash
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

Finally, let's create our first commit.

```bash
$ git add .
$ git commit -m "Initial commit with ZAMM"
[main (root-commit) 7a31990] Initial commit with ZAMM
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

## Confirmation

Let's make sure that we've committed everything by making sure that the working tree is clean:

```bash
$ git status
On branch main
nothing to commit, working tree clean
```

That's all! **Don't take any more steps** because the task is now done!