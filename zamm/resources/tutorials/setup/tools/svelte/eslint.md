# Setting up eslint for Svelte

First, install `eslint` and associated dependencies for interop with Svelte:

```bash
$ yarn add eslint @typescript-eslint/eslint-plugin @typescript-eslint/parser svelte-eslint-parser eslint-plugin-svelte --dev
```

Then, you can configure eslint for Svelte by copying the below config to `.eslintrc.yaml`:

```yaml
env:
  browser: true
  node: true
  es6: true
extends:
- plugin:svelte/recommended
- eslint:recommended
- plugin:@typescript-eslint/recommended
overrides:
- files: ['*.svelte']
  parser: svelte-eslint-parser
  parserOptions:
    parser: '@typescript-eslint/parser'
rules:
  max-len:
    - error
    - code: 88
  no-inner-declarations: off
  '@typescript-eslint/no-unused-vars':
    [error, { argsIgnorePattern: ^_, varsIgnorePattern: ^_ }]
ignorePatterns: [build/, dist/]
```

The config is based on [the one at awesome-sveltekit](https://github.com/janosh/awesome-sveltekit/blob/ea85d85d/site/.eslintrc.yml). `no-inner-declarations` is turned off because it doesn't understand that Svelte functions are already not at root. `max-len` is for compatibility with Python black.

## Errors

we find [this issue](https://github.com/sveltejs/kit/issues/5125), which links to [this answer](https://stackoverflow.com/a/74357681), which notes that the Eslint rule is redundant when we're using TypeScript. So edit `src-svelte/.eslintrc.yaml` to disable the rule:

```yaml
...
rules:
  ...
  no-undef: off
```
