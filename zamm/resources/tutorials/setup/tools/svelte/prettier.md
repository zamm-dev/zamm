# Setting up prettier for Svelte

First install prettier along with [plugins](https://github.com/sveltejs/prettier-plugin-svelte):

```bash
$ yarn add prettier prettier-plugin-svelte --dev
```

Then define `prettier.rc`

```json
{
    "plugins": ["prettier-plugin-svelte"]
}
```

Now this should work:

```bash
$ yarn prettier --write --plugin prettier-plugin-svelte src/
yarn run v1.22.19
$ /home/amos/Documents/ui/zamm/node_modules/.bin/prettier --write --plugin prettier-plugin-svelte src/
src/App.svelte 352ms
src/lib/Greet.svelte 81ms
src/main.ts 309ms
src/styles.css 52ms
src/vite-env.d.ts 9ms
Done in 1.52s.
```

Note that we have to specify `--plugin prettier-plugin-svelte` despite already including it in `prettier.rc` due to [this bug](https://github.com/prettier/prettier/issues/15079). It appears the file still gets formatted if you pass in the file directly -- e.g. `yarn prettier --write src/lib/Greet.svelte` -- so it's optional for pre-commit.

If you have `eslint` set up as well, make sure they don't clash by installing `eslint-config-prettier`:

```bash
$ yarn add eslint-config-prettier --dev
yarn add v1.22.19
[1/4] Resolving packages...
[2/4] Fetching packages...
...
└─ eslint-config-prettier@9.0.0
info All dependencies
└─ eslint-config-prettier@9.0.0
Done in 3.93s.
```

Then add `prettier` to the eslint config, say at `.eslintrc.yaml`, from:

```yaml
...
extends:
  - plugin:svelte/recommended
  - eslint:recommended
...
```

to

```yaml
...
extends:
  - prettier
  - plugin:svelte/recommended
  - eslint:recommended
...
```
