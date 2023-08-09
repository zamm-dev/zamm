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
        entry: cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged -- -Dwarnings
        language: system
        types: [rust]
        pass_filenames: false
```
