.PHONY: format lint test tests clean release

target/release/zamm: ./Cargo.toml ../src-svelte/build $(shell find . -type f \( -name "*.rs" \) -not -path "./target/*")
	cargo build --release --features custom-protocol
	touch target/release/zamm

all: format lint test build

format:
	cargo fmt

lint:
	cargo clippy -- -Dwarnings

test: tests
tests:
	cargo test -- --include-ignored --test-threads=1

test-files:
	python3 api/sample-database-writes/many-api-calls/generate.py

clean:
	cargo clean
