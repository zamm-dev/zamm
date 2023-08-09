# Setting up Ruff for pre-commit

You should set `--fix` and `--exit-non-zero-on-fix` in general. If your Python project is inside a subdirectory in the current monorepo, then also set `--config <subdir>/pyproject.toml`.

```yaml
...
    hooks:
      - id: ruff
        args:
          - --fix
          - --exit-non-zero-on-fix
          - --config
          - src-python/pyproject.toml
```
