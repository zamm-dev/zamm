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
        files: \.(json|yaml|html|js|ts|svelte)$
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

We would probably want to type-check `test.ts` files too, so:

```yaml
      - id: typecheck-svelte
        name: svelte-check
        entry: yarn svelte-check --fail-on-warnings
        language: system
        types: [file]
        files: \.(ts|svelte)$
        exclude: ^webdriver/
```

Note that `svelte-check` depends on the directory it's run in. So this may pass:

```bash
$ yarn svelte-check src-svelte/src/routes/homepage.test.ts
yarn run v1.22.19
$ /root/zamm/node_modules/.bin/svelte-check src-svelte/src/routes/homepage.test.ts

====================================
Loading svelte-check in workspace: /root/zamm
Getting Svelte diagnostics...

====================================
svelte-check found 0 errors and 0 warnings
Done in 3.43s.
```

But then this doesn't:

```bash
$ cd src-svelte
$ yarn svelte-check src/routes/homepage.test.ts       
yarn run v1.22.19
$ /root/zamm/node_modules/.bin/svelte-check src/routes/homepage.test.ts

====================================
Loading svelte-check in workspace: /root/zamm/src-svelte
Getting Svelte diagnostics...

/root/zamm/src-svelte/src/routes/homepage.test.ts:49:5
Error: Property 'source' is missing in type '{ value: string; }' but required in type 'ApiKey'. 
  const mockApiKeys: ApiKeys = {
    openai: {
      value: "0p3n41-4p1-k3y",


====================================
svelte-check found 1 error and 0 warnings in 1 file
error Command failed with exit code 1.
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

So we'll have to enter the `src-svelte` directory first. We can't do this with `cd` because it doesn't exist inside the `pre-commit` execution environment:

```
svelte-check.............................................................Failed
- hook id: typecheck-svelte
- exit code: 1

Executable `cd` not found
```

Instead, we do it by using the yarn workspace feature:

```yaml
      - id: typecheck-svelte
        name: svelte-check
        entry: yarn workspace gui svelte-check --fail-on-warnings
        language: system
        types: [file]
        files: \.(ts|svelte)$
        exclude: ^webdriver/
```
