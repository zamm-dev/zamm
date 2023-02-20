# General documentation

## Python projects

When creating a new project, do so with [the `poetry` tool](#poetry-tool-usage).

## Poetry tool usage

[`poetry`](https://python-poetry.org/) is the newest Python tool for packaging and dependency management that is in popular use.

### Project creation

To create a new project, run the command

```bash
poetry new <PROJECT_NAME>
```

where `<PROJECT_NAME>` is the name of the new project you want to start.

Source: https://python-poetry.org/docs/basic-usage/

**Success checklist**

1. Verify that the output looks like

    ```
    Created package <PACKAGE_NAME> in <PROJECT_NAME>
    ```

2. There should now be a new folder under the name of `<PACKAGE_NAME>` in the current directory. Go into that folder and verify that a Poetry environment can be activated there.

**Example Success**

To create a new project named `hello-world`, run the command

```bash
$ poetry new hello-world
Created package hello_world in hello-world
```

Now a folder named `hello_world` should be created. We enter the new project

```bash
$ cd hello_world
```

and follow instructions for [activating a virtual environment](#virtual-environment-activation) inside the project.

### Virtual environment management

Poetry automatically creates and manages virutalenvs for its projects.

#### Virtual environment activation

**Prerequisites checklist**

1. Make sure you're already inside the project folder

To activate the virutal env for the project, run the command

```bash
poetry shell
```

**Success checklist**

1. Verify that the output contains a line that looks like this:

    ```
    Spawning shell within /home/<USER_DIR>/.cache/pypoetry/virtualenvs/<POETRY_ENV>
    ```

**Example success**

```bash
$ poetry shell
Creating virtualenv hello-world-gMSUUyFe-py3.10 in /home/amos/.cache/pypoetry/virtualenvs
Spawning shell within /home/<USER_DIR>/.cache/pypoetry/virtualenvs/hello-world-gMSUUyFe-py3.10
```

#### Virtual environment cleanup

To remove a virtualenv, run the command

```bash
poetry env remove python
```

**Success checklist**

1. Verify that the output contains a line that looks like this:

    ```
    Deleted virtualenv: /home/<USER_DIR>/.cache/pypoetry/virtualenvs/<POETRY_ENV>
    ```
