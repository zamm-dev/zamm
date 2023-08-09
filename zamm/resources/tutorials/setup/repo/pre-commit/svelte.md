# Configuring Svelte for pre-commit

Add linting:

```yaml
- repo: https://github.com/pre-commit/mirrors-eslint
  rev: v8.46.0
  hooks:
  - id: eslint
    types: [file]
    args: [--fix, --max-warnings=0]
    files: \.(js|ts|svelte)$
    additional_dependencies:
    - eslint
    - svelte
    - typescript
    - eslint-plugin-svelte
    - eslint-config-prettier
    - '@typescript-eslint/eslint-plugin'
    - '@typescript-eslint/parser'
    - svelte-eslint-parser
```

`eslint-config-prettier` is if you have prettier set up as well. Note that for some reason, `types_or` in pre-commit does not work as expected. For example, this config:

```yaml
  - repo: https://github.com/pre-commit/mirrors-eslint
    rev: v8.46.0
    hooks:
      - id: eslint
        types_or: [file]
        args: [--fix, --max-warnings=0]
        files: \.(js|ts|svelte)$
        additional_dependencies:
          - eslint
          - svelte
          - typescript
          - eslint-plugin-svelte
          - eslint-config-prettier
          - '@typescript-eslint/eslint-plugin'
          - '@typescript-eslint/parser'
          - svelte-eslint-parser
```

results in

```bash
$ pre-commit run eslint -v
eslint...................................................................Passed
- hook id: eslint
- duration: 4.72s
```

But change `types_or` to `types`:

```yaml
  - repo: https://github.com/pre-commit/mirrors-eslint
    rev: v8.46.0
    hooks:
      - id: eslint
        types: [file]
        args: [--fix, --max-warnings=0]
        files: \.(js|ts|svelte)$
        additional_dependencies:
          - eslint
          - svelte
          - typescript
          - eslint-plugin-svelte
          - eslint-config-prettier
          - '@typescript-eslint/eslint-plugin'
          - '@typescript-eslint/parser'
          - svelte-eslint-parser
```

And you get, as expected, lint failures:

```yaml
$ pre-commit run eslint -v
eslint...................................................................Failed
- hook id: eslint
- duration: 4.25s
- exit code: 1


/home/amos/Documents/ui/zamm/src/lib/Greet.svelte
  14:14  warning  'asdf' is defined but never used  @typescript-eslint/no-unused-vars

✖ 1 problem (0 errors, 1 warning)

ESLint found too many warnings (maximum: 0).

(zamm-py3.11) ➜  zamm git:(main) ✗ git add .               
(zamm-py3.11) ➜  zamm git:(main) ✗ pre-commit run eslint -v
eslint...................................................................Passed
- hook id: eslint
- duration: 4.72s
```

Add formatting:

```yaml
repos:
  ...
  - repo: https://github.com/pre-commit/mirrors-prettier
    rev: v3.0.1
    hooks:
      - id: prettier
        args: [--write, --plugin, prettier-plugin-svelte]
        additional_dependencies:
          - prettier
          - prettier-plugin-svelte
          - svelte
```

Add type-checking:

```yaml
repos:
  ...
  - repo: local
    hooks:
    ...
      - id: typecheck-svelte
        name: svelte-check
        entry: yarn svelte-check --fail-on-warnings
        language: system
        types: [svelte]
```
