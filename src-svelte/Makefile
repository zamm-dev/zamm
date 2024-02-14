.PHONY: format lint test

build: forks/neodrag/packages/svelte/dist $(shell find src -type f \( -name "*.svelte" -o -name "*.js" -o -name "*.ts" -o -name "*.css" -o -name "*.html" \) -not -path "*/node_modules/*")
	yarn install --frozen-lockfile && yarn svelte-kit sync && yarn build

forks/neodrag/packages/core/dist: forks/neodrag/packages/core/src/*
	cd forks/neodrag/packages/core && pnpm install && pnpm compile

forks/neodrag/packages/svelte/dist: forks/neodrag/packages/core/dist forks/neodrag/packages/svelte/src/*
	cd forks/neodrag/packages/svelte && pnpm install && pnpm compile

local-screenshots:
	mkdir -p screenshots/local
	cp -r screenshots/baseline screenshots/local/baseline

update-local-screenshots:
	cp -r screenshots/local/testing/actual/* screenshots/local/baseline/
	cp -r screenshots/local/testing/actual/* screenshots/baseline/

test:
	yarn test

format:
	yarn prettier --write --plugin prettier-plugin-svelte src/

lint:
	yarn svelte-check --fail-on-warnings
	yarn eslint --fix src/ --ext .js,.ts,.svelte

clean:
	rm -rf build node_modules ../node_modules forks/neodrag/packages/svelte/dist/dist
