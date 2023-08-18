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
ignorePatterns: [build/, dist/]
```

The config is based on [the one at awesome-sveltekit](https://github.com/janosh/awesome-sveltekit/blob/ea85d85d/site/.eslintrc.yml). `no-inner-declarations` is turned off because it doesn't understand that Svelte functions are already not at root. `max-len` is for compatibility with Python black.
