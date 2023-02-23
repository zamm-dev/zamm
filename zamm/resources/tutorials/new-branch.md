# Creating a new `git` branch for a task

Say you want to do the following task:

> Start a new branch for renaming the test workflow

You can do so by following these steps:

```bash
$ git checkout main
Switched to branch 'main'
Your branch is up to date with 'origin/main'.
$ git pull
remote: Enumerating objects: 1, done.
remote: Counting objects: 100% (1/1), done.
remote: Total 1 (delta 0), reused 0 (delta 0), pack-reused 0
Unpacking objects: 100% (1/1), 670 bytes | 670.00 KiB/s, done.
From github.com:amosjyng/zamm
   d766411..301f738  main       -> origin/main
Updating d766411..301f738
Fast-forward
 Makefile | 10 +++++++---
 1 file changed, 7 insertions(+), 3 deletions(-)
```

Now we pick a new branch name. For example, since this particular task involves "renaming the test workflow", we'll go with `rename-test-workflow`.

```bash
$ git checkout -b rename-test-workflow
Switched to a new branch 'rename-test-workflow'
```

That's all! **Don't take any more steps** because the task is now done!
