# Poetry

## Forcibly adding a dependency

As shown in [this answer](https://stackoverflow.com/a/60579329) which references [this discussion](https://github.com/python-poetry/poetry/issues/697#issuecomment-470431668), this is not possible. Instead, you can use pip to install conflicting dependencies instead. In order to do it inside the poetry environment, run

```bash
$ poetry run pip install <package>
```
