# Using `thiserror` to create custom error types

First install it:

```bash
$ cargo add thiserror
```

Then use it to define your custom errors:

```rust
#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Failed to spawn sidecar at {expected_path}: {tauri_error}")]
    SidecarSpawnError {
        expected_path: String, tauri_error: tauri::api::Error,}
}
```

and construct your custom errors:

```rust
        let expected_binary_path = relative_command_path(command.into())?;
        let (mut rx, mut _child) = match Command::new_sidecar(command)?
            .args(args)
            .spawn() {
                Ok((rx, child)) => (rx, child),
                Err(err) => {
                    return Err(Error::SidecarSpawnError {
                        expected_path: expected_binary_path,
                        tauri_error: err,
                    }.into())
                }
            };
```
