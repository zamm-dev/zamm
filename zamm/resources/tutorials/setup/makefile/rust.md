# Setting up `make` for Rust projects

`make` can help serve as a unified format/lint/test entry point.

The very first thing we do is to create a `Makefile` **in the current directory** (so edit `./Makefile`) that looks like this:

```Makefile
.PHONY: format lint test tests clean release

all: format lint test build

format:
	cargo fmt

lint:
	cargo clippy -- -Dwarnings

build:
	cargo build --release

test: tests
tests:
	cargo test

clean:
	cargo clean
```

We set `-Dwarnings` for clippy so that it will return a non-zero exit code for downstream consumption, e.g. for the pre-commit hook so that it will fail before committing if there are outstanding warnings.

## Confirmation

As usual, we check that we've configured `make` successfully by running it:

```bash
$ make
cargo fmt
cargo clippy
    Checking zamm v0.0.0 (/home/amos/Documents/ui/zamm/src-tauri)
warning: unneeded `return` statement
...
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Note that **it's okay if some of the steps fail**. The only goal here is to make sure that we've set up `make` successfully, not whether or not the make commands actually work. This would be an example of an incorrect Makefile setup which necessitates further probing:

```bash
$ make
make: *** No targets specified and no makefile found.  Stop.
```

Otherwise, **DECLARE THE TASK DONE**. That's all! **Don't take any more steps** because the task is now done!