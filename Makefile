.PHONY: rust-format rust-lint quicktype

BUILD_IMAGE = ghcr.io/amosjyng/zamm:v0.0.0-build
CURRENT_DIR = $(shell pwd)

build: svelte rust
	cargo tauri build $(ARGS)

mac: svelte rust
	cargo tauri build --target universal-apple-darwin --verbose

copy-docker-deps:
	mv -n /tmp/forks/async-openai/* ./forks/async-openai/
	mv -n /tmp/forks/rvcr/* ./forks/rvcr/
	mv -n /tmp/dependencies/src-svelte/forks/neodrag/packages/svelte/dist ./src-svelte/forks/neodrag/packages/svelte/
	mv -n /tmp/dependencies/node_modules ./
	mv -n /tmp/dependencies/src-svelte/node_modules ./src-svelte/
	mv -n /tmp/dependencies/target ./src-tauri/

build-docker:
	docker run --rm -v $(CURRENT_DIR):/zamm -w /zamm $(BUILD_IMAGE) make copy-docker-deps build ARGS=$(ARGS)

icon:
	yarn tauri icon src-tauri/icons/icon.png

docker:
	docker build . -t $(BUILD_IMAGE)
	docker push $(BUILD_IMAGE)

e2e-test: svelte rust
	yarn e2e-test

test: svelte rust
	cd src-svelte && make test
	cd src-tauri && make test
	yarn e2e-test

quicktype:
	yarn quicktype src-tauri/api/sample-call-schema.json -s schema -o src-tauri/src/sample_call.rs --visibility public --derive-debug
	yarn quicktype src-tauri/api/sample-call-schema.json -s schema -o src-svelte/src/lib/sample-call.ts

rust-format:
	cd src-tauri && make format

rust-lint:
	cd src-tauri && make lint

rust:
	cd src-tauri && make

svelte-format:
	cd src-svelte && make format

svelte-lint:
	cd src-svelte && make lint

svelte:
	cd src-svelte && make

clean:
	cd src-svelte && make clean
	cd src-tauri && make clean
