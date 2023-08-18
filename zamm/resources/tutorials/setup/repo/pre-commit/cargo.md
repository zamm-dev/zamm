# Setting up Cargo for pre-commit

`--manifest-path` is only needed if your project is in a subdirectory.

```yaml
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt --manifest-path src-tauri/Cargo.toml --
        language: system
        types: [rust]
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings
        language: system
        types: [rust]
        pass_filenames: false
```

If there are some warnings emitted by libraries that don't cause clippy to return a nonzero exit code, you can simulate that manually by creating `clippy.sh`:

```sh
#!/usr/bin/bash

clippy_output=$(cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings)

if [[ $clippy_output =~ "warning" ]]; then
  exit 0;
else
  exit 1;
fi
```

and then defining a pre-commit hook that calls that instead (make sure to `chmod +x` the script as needed)

```yaml
      - id: cargo-clippy
        name: cargo clippy
        entry: src-tauri/clippy.sh
        language: system
        types: [rust]
        pass_filenames: false
```
