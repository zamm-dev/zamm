# Setting up `make` for Rust projects

`make` can help serve as a unified format/lint/test entry point.

The very first thing we do is to create a `Makefile` **in the current directory** (so edit `./Makefile`) that looks like this:

```Makefile
.PHONY: format lint test

build: $(shell find src -type f \( -name "*.svelte" -o -name "*.js" -o -name "*.ts" -o -name "*.html" \) -not -path "*/node_modules/*")
	yarn && yarn svelte-kit sync && yarn build

test:
	yarn test

format:
	yarn prettier --write --plugin prettier-plugin-svelte src/

lint:
	yarn svelte-check --fail-on-warnings
	yarn eslint --fix src/ --ext .js,.ts,.svelte
```

## Confirmation

As usual, we check that we've configured `make` successfully by running it:

```bash
$ make
yarn prettier --write --plugin prettier-plugin-svelte src/
yarn run v1.22.19
$ /home/amos/Documents/ui/zamm/node_modules/.bin/prettier --write --plugin prettier-plugin-svelte src/
...
yarn run v1.22.19
$ /home/amos/Documents/ui/zamm/node_modules/.bin/eslint --fix src/
Done in 4.03s.
```

Note that **it's okay if some of the steps fail**. The only goal here is to make sure that we've set up `make` successfully, not whether or not the make commands actually work. This would be an example of an incorrect Makefile setup which necessitates further probing:

```bash
$ make
make: *** No targets specified and no makefile found.  Stop.
```

Otherwise, **DECLARE THE TASK DONE**. That's all! **Don't take any more steps** because the task is now done!