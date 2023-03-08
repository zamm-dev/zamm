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