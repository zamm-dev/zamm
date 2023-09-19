# Using anyhow

First install it:

```bash
$ cargo add anyhow
```

Then use it to return generic results:

```rust
use anyhow::Result;

  fn execute<I, S>(command: &str, args: I) -> Result<String>
    where
      I: IntoIterator<Item = S>,
      S: AsRef<str>,
    {
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

        
        // https://stackoverflow.com/a/52521592
        let stdout = executor::block_on(tauri::async_runtime::spawn(async move {
            let mut output = String::new();
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    output.push_str(&line);
                } else if let CommandEvent::Error(line) = event {
                    output.push_str(&line);
                }
            }
            output
        }))?;

        Ok(stdout)
    }
```
