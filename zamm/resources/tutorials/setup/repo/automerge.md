# Enabling automerge on Github

The first command we run is to find out what our remote is.

```bash
$ git remote -v
origin  git@github.com:amosjyng/langchain-visualizer.git (fetch)
origin  git@github.com:amosjyng/langchain-visualizer.git (push)
```

Go to the project settings page. Given the above remote, the URL for the settings page will be `https://github.com/amosjyng/langchain-visualizer/settings`.

Then click on the checkboxes named "Allow auto-merge" and "Automatically delete head branches".
