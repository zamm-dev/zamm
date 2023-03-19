Say you want to do the following task:

> Set up readthedocs

You can do so by following these steps:

## Sphinx setup

You proceed to use the terminal:

```bash
$ poetry add --group docs sphinx
Using version ^6.1.3 for sphinx

Updating dependencies
Resolving dependencies... (15.2s)

Writing lock file

Package operations: 12 installs, 0 updates, 0 removals

  • Installing markupsafe (2.1.2)
  • Installing alabaster (0.7.13)
  • Installing babel (2.12.1)
  • Installing imagesize (1.4.1)
  • Installing jinja2 (3.1.2)
  • Installing sphinxcontrib-applehelp (1.0.4)
  • Installing sphinxcontrib-devhelp (1.0.2)
  • Installing sphinxcontrib-htmlhelp (2.0.1)
  • Installing sphinxcontrib-jsmath (1.0.1)
  • Installing sphinxcontrib-qthelp (1.0.3)
  • Installing sphinxcontrib-serializinghtml (1.1.5)
  • Installing sphinx (6.1.3)
```

You note that run `poetry run sphinx-quickstart`, but outside of here because the interactive terminal is currently not supported. Alternatively, use the commandline to create it in one go:

You proceed to use the terminal:

```bash
$ poetry run sphinx-quickstart -q -p langchain-contrib -a "Amos Ng" -v 0.0.1 --ext-autodoc --makefile docs/

Finished: An initial directory structure has been created.

You should now populate your master file /home/amos/projects/gpt-experiments/langchain-contrib/docs/index.rst and create other documentation
source files. Use the Makefile to build the docs, like so:
   make builder
where "builder" is one of the supported builders, e.g. html, latex or linkcheck.
$ cd docs
$ poetry shell
Spawning shell within /home/amos/.cache/pypoetry/virtualenvs/langchain-contrib-mTbtaW20-py3.11
$ make
/bin/sh: 1: sphinx-build: not found
make: *** [Makefile:20: /home/amos/.local/share/zamm/makefiles/general.mk] Error 127
```

You note that can't sphinx build because poetry shell not supported yet

Follow instructions at https://stackoverflow.com/a/29357747

```bash
$ sphinx-apidoc -o modules/ ../langchain_contrib
```

## Version sync

To automatically sync the built docs version with the poetry version, install `toml`:

```bash
$ poetry add --group docs toml
```

Then edit `conf.py` to read in `pyproject.toml`. If it currently looks like:

```python
"""Configuration file for the Sphinx documentation builder."""

# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "langchain-contrib"
copyright = "2023, Amos Ng"
author = "Amos Ng"

version = "0.0.1"
release = "0.0.1"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

...
```

then change it to

```python
"""Configuration file for the Sphinx documentation builder."""

# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "langchain-contrib"
copyright = "2023, Amos Ng"
author = "Amos Ng"

import toml

with open("../pyproject.toml") as f:
    data = toml.load(f)

version = data["tool"]["poetry"]["version"]
release = version

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

...
```

Finally, edit `docs/requirements.txt` to add in the new `toml` dependency. If it was like this before:

```
nbsphinx==0.9.1
sphinx_book_theme==1.0.0
```

then it should be like this now:

```
nbsphinx==0.9.1
sphinx_book_theme==1.0.0
toml==0.10.2
```

## Auto-build

Install `sphinx-autobuild` for fast documentation editing.

```bash
$ poetry add --group docs sphinx-autobuild
```

Edit `./Makefile`. If it looks like

```
# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line, and also
# from the environment for the first two.
SPHINXOPTS    ?=
SPHINXBUILD   ?= sphinx-build
SOURCEDIR     = .
BUILDDIR      = _build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
```

change it to add a new `livehtml` target:

```
# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line, and also
# from the environment for the first two.
SPHINXOPTS    ?=
SPHINXBUILD   ?= sphinx-build
SOURCEDIR     = .
BUILDDIR      = _build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

livehtml:
	sphinx-autobuild "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
```

## Jupyter notebook support

Install `pandoc` and then `nbsphinx`:

```bash
$ sudo apt-get install pandoc
$ poetry add --group docs nbsphinx
```

Then edit `conf.py`. If it looks like

```python
"""Configuration file for the Sphinx documentation builder."""

# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "langchain-contrib"
copyright = "2023, Amos Ng"
author = "Amos Ng"

version = "0.0.1"
release = "0.0.1"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]
```

add the new extension in:

```python
"""Configuration file for the Sphinx documentation builder."""

# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "langchain-contrib"
copyright = "2023, Amos Ng"
author = "Amos Ng"

version = "0.0.1"
release = "0.0.1"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
    "nbsphinx",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]
```

Also edit `.readthedocs.yaml`:

```yaml
# .readthedocs.yaml
# Read the Docs configuration file
# See https://docs.readthedocs.io/en/stable/config-file/v2.html for details

# Required
version: 2

# Set the version of Python and other tools you might need
build:
  os: ubuntu-22.04
  tools:
    python: "3.11"
    # You can also specify other tool versions:
    # nodejs: "19"
    # rust: "1.64"
    # golang: "1.19"

# Build documentation in the docs/ directory with Sphinx
sphinx:
   configuration: docs/conf.py

# If using Sphinx, optionally build your docs in additional formats such as PDF
# formats:
#    - pdf

# Optionally declare the Python requirements required to build your docs
python:
   install:
   - requirements: docs/requirements.txt
```

Then create `docs/requirements.txt` with the version of nbsphinx installed above:

```
nbsphinx==0.9.1
```

## Theme

To use the same theme as langchain, install `sphinx-book-theme`:

```bash
$ poetry add --group docs sphinx-book-theme
  • Installing accessible-pygments (0.0.3)
  • Installing pydata-sphinx-theme (0.13.1)
  • Installing sphinx-book-theme (1.0.0)
```

Then edit `conf.py` to use the theme. If it looks like

```python
"""Configuration file for the Sphinx documentation builder."""

# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "langchain-contrib"
copyright = "2023, Amos Ng"
author = "Amos Ng"

version = "0.0.1"
release = "0.0.1"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
    "nbsphinx",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "alabaster"
html_static_path = ["_static"]
```

then change it to

```python
"""Configuration file for the Sphinx documentation builder."""

# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "langchain-contrib"
copyright = "2023, Amos Ng"
author = "Amos Ng"

version = "0.0.1"
release = "0.0.1"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
    "nbsphinx",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "sphinx_book_theme"

html_theme_options = {
    "path_to_docs": "docs",
    "repository_url": "https://github.com/amosjyng/langchain-contrib",
    "use_repository_button": True,
}

html_context = {
    "display_github": True,  # Integrate GitHub
    "github_user": "amosjyng",  # Username
    "github_repo": "langchain-contrib",  # Repo name
    "github_version": "main",  # Version
    "conf_py_path": "/docs/",  # Path in the checkout to the docs root
}

html_static_path = ["_static"]
```

Edit `docs/requirements.txt` again to add this new theme. If it current looks like

```
nbsphinx==0.9.1
```

then add the theme:

```
nbsphinx==0.9.1
sphinx_book_theme==1.0.0
```

If you see this afterwards:

> Color theme None not found by pygments

know that it's a [known issue](https://github.com/executablebooks/sphinx-book-theme/issues/690).

That's all! **Don't take any more steps** because the task is now done!