# Setting up PyPI

First login to PyPI: [https://pypi.org/account/login/](https://pypi.org/account/login/).

Then, visit the account page: [https://pypi.org/manage/account/](https://pypi.org/manage/account/). Go to the "API tokens" section and click on "Add API token". Follow the instructions, then copy the API token and add the following environment variables to your shell init script:

```bash
export PYPI_USERNAME="__token__"
export PYPI_PASSWORD=<your new PyPI key>
```
