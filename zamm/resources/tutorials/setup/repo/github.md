# Uploading a Git repo to Github

Github is a popular place to upload and share Git repos with others. So say you want to do the following task:

> Upload the current Git repo to Github

You can do so by following these steps:


**The very first command we run** is to find out what the name of the current project is.

```bash
$ pwd
/tmp/zamm/scratch/hello-world
```

In this case, we are in a folder called `hello-world`, so we will call the Github API to create a project with the corresponding name. **Your folder name will be different**, so make sure to run the above command first to figure out what your project name is before running the following command to create a project on Github with the same name:

```bash
$ curl -L -X POST -H "Accept: application/vnd.github+json" -H "Authorization: Bearer $GITHUB_TOKEN" -H "X-GitHub-Api-Version: 2022-11-28" https://api.github.com/user/repos -d '{"name":"hello-world","private":true}'
{
  "id": 611075492,
  "node_id": "R_kgDOJGxFpA",
  "name": "hello-world",
  "full_name": "amosjyng/hello-world",
  "private": true,
  ...
  "git_url": "git://github.com/amosjyng/hello-world.git",
  ...
  "merge_commit_title": "MERGE_MESSAGE",
  "network_count": 0,
  "subscribers_count": 0
}
```

Note that the `git_url` key has a value of `git://github.com/amosjyng/hello-world.git`. So that is the URL we will now set the remote to:

```bash
$ git remote add origin git@github.com:amosjyng/hello-world.git
$ git push -u origin main
Enumerating objects: 5, done.
Counting objects: 100% (5/5), done.
Delta compression using up to 16 threads
Compressing objects: 100% (3/3), done.
Writing objects: 100% (5/5), 746 bytes | 746.00 KiB/s, done.
Total 5 (delta 0), reused 0 (delta 0)
To github.com:amosjyng/hello-world.git
 * [new branch]      main -> main
Branch 'main' set up to track remote branch 'main' from 'origin'.
```

## Confirmation

To check that we have set everything up correctly, let's try pulling. It should say that everything is already up to date because we have only just uploaded the repo.

```bash
$ git pull
Already up to date.
```

**Stop now** because "Already up to date" exists in the terminal output. **Don't take any more steps** because the task is now done!

## Secret environment variables in external PRs

If tests on a pull request are failing because secret environment variables are not being shown, even though you clearly have them defined on a page such as https://github.com/YOUR_USERNAME/YOUR_REPO/settings/secrets/actions, this is likely because of GitHub's security mechanisms.

[This answer](https://stackoverflow.com/a/73867674) appears to suggest using `pull_request_target` instead. However, it's not clear how this would work.
