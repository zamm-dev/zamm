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

If there are some warnings emitted by libraries that don't cause clippy to return a nonzero exit code (for an example, see [`diesel.md`](/zamm/resources/tutorials/libraries/rust/diesel.md)), you can simulate that manually by creating `clippy.sh`:

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

You can make the if statement shorter with

```bash
if [[ $clippy_output == *"warning"* ]]; then
  exit 1;
fi
```

To check for warnings specific to your project and not any forks (as was needed in [`chat.md`](/ui/chat.md)), do:

```bash
...
zamm_output=$(echo "$clippy_output" | awk '/Checking zamm /{flag=1; next} flag')

if [[ $zamm_output == *"warning"* ]]; then
  exit 1;
fi

```

To avoid cluttering the output every time you run `git commit`, you can do:

```bash
clippy_output=$(cargo clippy --manifest-path src-tauri/Cargo.toml --fix --allow-dirty --allow-staged --all-targets --all-features -- -Dwarnings 2>&1)
zamm_output=$(echo "$clippy_output" | awk '/Checking zamm /{flag=1; next} flag')
echo "$zamm_output"
```
