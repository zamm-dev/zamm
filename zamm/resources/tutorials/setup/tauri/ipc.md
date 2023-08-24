# IPC with the frontend

An example has already been set up for you with the default Tauri app setup.

## Adding error support

To allow it to fail, change it from

```python
#[tauri::command]
#[specta]
pub fn greet(name: &str) -> String {
    greet_helper(&SidecarExecutorImpl {}, name)
}
```

to

```python
fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> Result<String> {
    let result = process::<GreetArgs, GreetResponse>(
        t,
        "zamm-python",
        "greet",
        &GreetArgs { name: name.into() },
    )?;
    let greeting = result.greeting;
    Ok(format!("{greeting} via Rust"))
}

#[tauri::command]
#[specta]
pub fn greet(name: &str) -> Result<String> {
    greet_helper(&SidecarExecutorImpl {}, name)
}
```

You may get this error:

```
error[E0599]: the method `blocking_kind` exists for reference `&Result<String, Error>`, but its trait bounds were not satisfied
   --> src/commands.rs:100:1
    |
100 | #[tauri::command]
    | ^^^^^^^^^^^^^^^^^ method cannot be called on `&Result<String, Error>` due to unsatisfied trait bounds
    |
   ::: /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.75/src/lib.rs:374:1
    |
374 | pub struct Error {
    | ---------------- doesn't satisfy `anyhow::Error: Into<InvokeError>`
    |
   ::: /root/.asdf/installs/rust/1.71.1/toolchains/1.71.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:502:1
    |
502 | pub enum Result<T, E> {
    | ---------------------
    | |
    | doesn't satisfy `_: ResultKind`
    | doesn't satisfy `_: Serialize`
    |
   ::: src/main.rs:34:25
    |
34  |         .invoke_handler(tauri::generate_handler![greet])
    |                         ------------------------------- in this macro invocation
    |
    = note: the following trait bounds were not satisfied:
            `anyhow::Error: Into<InvokeError>`
            which is required by `std::result::Result<std::string::String, anyhow::Error>: tauri::command::private::ResultKind`
            `std::result::Result<std::string::String, anyhow::Error>: Serialize`
            which is required by `&std::result::Result<std::string::String, anyhow::Error>: tauri::command::private::SerializeKind`
    = note: this error originates in the macro `__cmd__greet` which comes from the expansion of the macro `tauri::generate_handler` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0599`.
```

As mentioned [here](https://github.com/tauri-apps/tauri/discussions/3913), we'll follow the instructions [here](https://jonaskruckenberg.github.io/tauri-docs-wip/development/inter-process-communication.html#error-handling) to fix this problem. We'll follow the idiomatic recommended approach of creating our own custom error. We already have one, so we just extend it.

We define a custom error type first:

```rust
#[derive(Debug)]
pub struct SidecarJsonError {
    request: Vec<String>,
    response: String,
    source: serde_json::Error,
}

impl std::error::Error for SidecarJsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for SidecarJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut json_err_msg = String::new();

        json_err_msg.push_str("Failed to parse sidecar JSON.\n");

        json_err_msg.push_str("Request: ");
        for arg in self.request.iter() {
            json_err_msg.push_str(arg);
            json_err_msg.push_str(" ");
        }
        json_err_msg.push_str("\n");

        json_err_msg.push_str("Response: ");
        json_err_msg.push_str(&self.response);
        json_err_msg.push_str("\n");

        json_err_msg.push_str("Error: ");
        json_err_msg.push_str(&self.source.to_string());
        write!(f, "{}", json_err_msg)
    }
}
```

We cannot do this inside our regular error enum:

```rust
    #[error("Failed to parse sidecar JSON.
Request: {request}.
Response: {response}.
Error: {source}")]
    SidecarJsonError {
        request: Vec<String>,
        response: String,
        source: serde_json::Error,
    },
```

because this results in the error

```
error[E0599]: the method `as_display` exists for reference `&Vec<String>`, but its trait bounds were not satisfied
   --> src/commands.rs:32:13
    |
32  |       #[error("Failed to parse sidecar JSON.
    |  _____________^
33  | | Request: {request}.
34  | | Response: {response}.
35  | | Error: {source}")]
    | |________________^ method cannot be called on `&Vec<String>` due to unsatisfied trait bounds
    |
   ::: /root/.asdf/installs/rust/1.71.1/toolchains/1.71.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:396:1
    |
396 |   pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    |   ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<std::string::String>: std::fmt::Display`
    |
    = note: the following trait bounds were not satisfied:
            `Vec<std::string::String>: std::fmt::Display`
            which is required by `&Vec<std::string::String>: DisplayAsDisplay`

For more information about this error, try `rustc --explain E0599`.
```

Instead, we define it with our custom one above:

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to spawn sidecar at {expected_path}: {tauri_error}")]
    SidecarSpawn {
        expected_path: String,
        tauri_error: tauri::api::Error,
    },
    #[error(transparent)]
    SidecarResponse {
        #[from]
        source: SidecarResponseError,
    },
    #[error(transparent)]
    Serde {
        #[from]
        source: serde_json::Error,
    },
    #[error(transparent)]
    Tauri {
        #[from]
        source: tauri::Error,
    },
    #[error("Unknown failure: {source}")]
    Other {
        #[from]
        source: anyhow::Error,
    },
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type ZammResult<T> = std::result::Result<T, Error>;
```

and use the new result everywhere:

```rust
#[automock]
trait SidecarExecutor {
    #[allow(clippy::needless_lifetimes)]
    fn execute<'a>(&self, command: &str, args: &[&'a str]) -> ZammResult<String>;
}

struct SidecarExecutorImpl;

impl SidecarExecutor for SidecarExecutorImpl {
    fn execute(&self, command: &str, args: &[&str]) -> ZammResult<String> {
        let expected_binary_path = relative_command_path(command.into())?;
        let (mut rx, mut _child) =
            match Command::new_sidecar(command)?.args(args).spawn() {
                Ok((rx, child)) => (rx, child),
                Err(err) => {
                    return Err(Error::SidecarSpawn {
                        expected_path: expected_binary_path,
                        tauri_error: err,
                    })
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
}

fn process<T: Serialize, U: for<'de> Deserialize<'de>>(
    s: &impl SidecarExecutor,
    binary: &str,
    command: &str,
    input: &T,
) -> ZammResult<U> {
    let input_json = serde_json::to_string(input)?;
    let result_json = s.execute(binary, &[command, &input_json])?;
    let response: U = match serde_json::from_str(&result_json) {
        Ok(response) => response,
        Err(err) => {
            return Err(SidecarResponseError {
                request: vec![binary.into(), command.into(), input_json],
                response: result_json,
                source: err,
            }
            .into())
        }
    };

    Ok(response)
}

fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> ZammResult<String> {
    let result = process::<GreetArgs, GreetResponse>(
        t,
        "zamm-python",
        "greet",
        &GreetArgs { name: name.into() },
    )?;
    let greeting = result.greeting;
    Ok(format!("{greeting} via Rust"))
}

#[tauri::command]
#[specta]
pub fn greet(name: &str) -> ZammResult<String> {
    match greet_helper(&SidecarExecutorImpl {}, name) {
        Ok(greeting) => Ok(greeting),
        Err(err) => {
            eprintln!("Greet error: {}", err);
            Err(err)
        }
    }
}
```

Now that `greet_helper` returns a result, we'll have to change our tests as well to unwrap it:

```rust
    fn check_greet_sample(file_prefix: &str, rust_input: &str, rust_result: &str) {
        ...

        let result = greet_helper(&mock, rust_input).unwrap();
        assert_eq!(result, rust_result);
    }
```

Next, we update the frontend:

```ts
  async function trigger_greet() {
    try {
      const result = await greet(name);
      greetMsg = result + " via TypeScript!";
    } catch (err) {
      greetMsg = "Sorry, we can't greet you right now. " + err;
    }
  }
```

Let's commit now, and try changing the command invocation to get it to fail:

```rust
    let result = process::<GreetArgs, GreetResponse>(
        t,
        "zamm-python",
        "greets",
        &GreetArgs { name: name.into() },
    )?;
```

Now we try running through the Greet page again, and verify that it fails. The Tauri backend prints out:

```
Greet error: Failed to parse sidecar JSON.
Request: zamm-python greets {"name":"asdf"} 
Response: 
Error: EOF while parsing a value at line 1 column 0
```

Note that it is unclear from this description which binary is actually being executed. We know theoretically that it should be the one in `target/debug/zamm-python`, but unfortunately the same binary also exists in `target/release/zamm-python`. Let's add the full path to our debug info:

```rust
        Err(err) => {
            let binary_path = relative_command_path(binary.into())?;
            return Err(SidecarResponseError {
                request: vec![binary_path, command.into(), input_json],
                response: result_json,
                source: err,
            }
            .into())
        }
```

When we run this again, we see that the entire request command is

```
/root/zamm/src-tauri/target/debug/zamm-python greets {"name":"asdf"}
```

Let's try to copy-paste the request to see if we can reproduce the exact error:

```
$ /root/zamm/src-tauri/target/debug/zamm-python greets {"name":"asdf"}  
Traceback (most recent call last):
  File "zamm/main.py", line 11, in <module>
  File "zamm/execution.py", line 15, in handle_commandline_args
KeyError: 'greets'
[1188716] Failed to execute script 'main' due to unhandled exception!
```

Somehow it seems we are missing the response. We quickly realize that it is because we're missing `stderr`. Let's define these new errors:

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    ...
    #[error("Sidecar command error event: {line}")]
    SidecarCommandErr {
        line: String,
    },
    #[error("Unexpected sidecar command event")]
    SidecarUnexpectedCommandEvent,
    ...
}

```

and update our executor accordingly:

```rust
impl SidecarExecutor for SidecarExecutorImpl {
    fn execute(&self, command: &str, args: &[&str]) -> ZammResult<String> {
        ...

        executor::block_on(tauri::async_runtime::spawn(async move {
            let mut output = String::new();
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        output.push_str(&line);
                        output.push_str("\n");
                    },
                    CommandEvent::Stderr(line) => {
                        output.push_str(&line);
                        output.push_str("\n");
                    },
                    CommandEvent::Error(line) => {
                        return Err(Error::SidecarCommandErr { line }.into())
                    },
                    CommandEvent::Terminated(_) => break,
                    _ => return Err(Error::SidecarUnexpectedCommandEvent.into()),
                }
            }
            Ok(output)
        }))?
}
```

Note that we print out `\n` now to account for multiline output. The output now:

```
Greet error: Failed to parse sidecar JSON.
Request: /root/zamm/src-tauri/target/debug/zamm-python greets {"name":"asdf"} 
Response: Traceback (most recent call last):
  File "zamm/main.py", line 11, in <module>
  File "zamm/execution.py", line 14, in handle_commandline_args
KeyError: 'greets'
[1209762] Failed to execute script 'main' due to unhandled exception!

Error: expected value at line 1 column 1
```

We finally have all the pieces of the error displayed to the terminal. Let's check the frontend:

```
Sorry, we can't greet you right now. Failed to parse sidecar JSON. Request: /root/zamm/src-tauri/target/debug/zamm-python greets {"name":"asdf"} Response: Traceback (most recent call last): File "zamm/main.py", line 11, in <module> File "zamm/execution.py", line 14, in handle_commandline_args KeyError: 'greets' [1209762] Failed to execute script 'main' due to unhandled exception! Error: expected value at line 1 column 1
```

We have successfully added error-handling so that the whole app doesn't crash when an IPC call fails. Remember to commit everything except for the code that triggers the failure!

## One command per file

Like what we did with the Python sidecar, we'll want to also make it easy to make one IPC command per Rust file. First, move `src-tauri/src/commands.rs` to `src-tauri/src/commands/mod.rs`, and then refactor that file into:

`src-tauri/src/commands/api.rs`:

```rust
use futures::executor;

use serde::{Deserialize, Serialize};

use tauri::api::process::{Command, CommandEvent};

use crate::commands::errors::{Error, SidecarResponseError, ZammResult};
use tauri_utils::platform;

use mockall::automock;

fn relative_command_path(command: String) -> tauri::Result<String> {
    match platform::current_exe()?.parent() {
        #[cfg(windows)]
        Some(exe_dir) => Ok(format!("{}\\{command}.exe", exe_dir.display())),
        #[cfg(not(windows))]
        Some(exe_dir) => Ok(format!("{}/{command}", exe_dir.display())),
        None => Err(tauri::api::Error::Command(
            "Could not evaluate executable dir".to_string(),
        )
        .into()),
    }
}

#[automock]
pub trait SidecarExecutor {
    #[allow(clippy::needless_lifetimes)]
    fn execute<'a>(&self, command: &str, args: &[&'a str]) -> ZammResult<String>;
}

pub struct SidecarExecutorImpl;

impl SidecarExecutor for SidecarExecutorImpl {
    fn execute(&self, command: &str, args: &[&str]) -> ZammResult<String> {
        let expected_binary_path = relative_command_path(command.into())?;
        let (mut rx, mut _child) =
            match Command::new_sidecar(command)?.args(args).spawn() {
                Ok((rx, child)) => (rx, child),
                Err(err) => {
                    return Err(Error::SidecarSpawn {
                        expected_path: expected_binary_path,
                        tauri_error: err,
                    })
                }
            };

        executor::block_on(tauri::async_runtime::spawn(async move {
            let mut output = String::new();
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        output.push_str(&line);
                        output.push('\n');
                    }
                    CommandEvent::Stderr(line) => {
                        output.push_str(&line);
                        output.push('\n');
                    }
                    CommandEvent::Error(line) => {
                        return Err(Error::SidecarCommandErr { line })
                    }
                    CommandEvent::Terminated(_) => break,
                    _ => return Err(Error::SidecarUnexpectedCommandEvent),
                }
            }
            Ok(output)
        }))?
    }
}

pub fn process<T: Serialize, U: for<'de> Deserialize<'de>>(
    s: &impl SidecarExecutor,
    binary: &str,
    command: &str,
    input: &T,
) -> ZammResult<U> {
    let input_json = serde_json::to_string(input)?;
    let result_json = s.execute(binary, &[command, &input_json])?;
    let response: U = match serde_json::from_str(&result_json) {
        Ok(response) => response,
        Err(err) => {
            let binary_path = relative_command_path(binary.into())?;
            return Err(SidecarResponseError {
                request: vec![binary_path, command.into(), input_json],
                response: result_json,
                source: err,
            }
            .into());
        }
    };

    Ok(response)
}

```

`src-tauri/src/commands/errors.rs`:

```rust
use std::fmt;

#[derive(Debug)]
pub struct SidecarResponseError {
    pub request: Vec<String>,
    pub response: String,
    pub source: serde_json::Error,
}

impl std::error::Error for SidecarResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for SidecarResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut json_err_msg = String::new();

        json_err_msg.push_str("Failed to parse sidecar JSON.\n");

        json_err_msg.push_str("Request: ");
        for arg in self.request.iter() {
            json_err_msg.push_str(arg);
            json_err_msg.push(' ');
        }
        json_err_msg.push('\n');

        json_err_msg.push_str("Response: ");
        json_err_msg.push_str(&self.response);
        json_err_msg.push('\n');

        json_err_msg.push_str("Error: ");
        json_err_msg.push_str(&self.source.to_string());
        write!(f, "{}", json_err_msg)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to spawn sidecar at {expected_path}: {tauri_error}")]
    SidecarSpawn {
        expected_path: String,
        tauri_error: tauri::api::Error,
    },
    #[error(transparent)]
    SidecarResponse {
        #[from]
        source: SidecarResponseError,
    },
    #[error("Sidecar command error event: {line}")]
    SidecarCommandErr { line: String },
    #[error("Unexpected sidecar command event")]
    SidecarUnexpectedCommandEvent,
    #[error(transparent)]
    Serde {
        #[from]
        source: serde_json::Error,
    },
    #[error(transparent)]
    Tauri {
        #[from]
        source: tauri::Error,
    },
    #[error("Unknown failure: {source}")]
    Other {
        #[from]
        source: anyhow::Error,
    },
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type ZammResult<T> = std::result::Result<T, Error>;

```

`src-tauri/src/commands/greet.rs`:

```rust
use crate::python_api::{GreetArgs, GreetResponse};

use specta::specta;

use crate::commands::api::{process, SidecarExecutor, SidecarExecutorImpl};
use crate::commands::errors::ZammResult;

fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> ZammResult<String> {
    let result = process::<GreetArgs, GreetResponse>(
        t,
        "zamm-python",
        "greet",
        &GreetArgs { name: name.into() },
    )?;
    let greeting = result.greeting;
    Ok(format!("{greeting} via Rust"))
}

#[tauri::command]
#[specta]
pub fn greet(name: &str) -> ZammResult<String> {
    match greet_helper(&SidecarExecutorImpl {}, name) {
        Ok(greeting) => Ok(greeting),
        Err(err) => {
            eprintln!("Greet error: {}", err);
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::api::MockSidecarExecutor;
    use crate::sample_call::SampleCall;

    use std::fs;

    fn parse_greet(args_str: &str) -> GreetArgs {
        serde_json::from_str(args_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    fn check_greet_sample(file_prefix: &str, rust_input: &str, rust_result: &str) {
        let greet_sample = read_sample(file_prefix);

        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(move |cmd, actual_cmd_args| {
                assert_eq!(cmd, "zamm-python");

                let expected_cmd_args = &greet_sample.request;
                assert_eq!(actual_cmd_args.len(), expected_cmd_args.len());
                assert_eq!(actual_cmd_args[0], expected_cmd_args[0]);

                let actual_greet_args = parse_greet(actual_cmd_args[1]);
                let expected_greet_args = parse_greet(&expected_cmd_args[1]);
                assert_eq!(actual_greet_args, expected_greet_args);

                true
            })
            .return_once(move |_, _| Ok(greet_sample.response));

        let result = greet_helper(&mock, rust_input).unwrap();
        assert_eq!(result, rust_result);
    }

    #[test]
    fn test_greet_name() {
        check_greet_sample(
            "../src-python/api/sample-calls/greet.yaml",
            "Test",
            "Hello, Test! You have been greeted from Python via Rust",
        );
    }

    #[test]
    fn test_greet_empty_name() {
        check_greet_sample(
            "../src-python/api/sample-calls/greet_empty.yaml",
            "",
            "Hello, ! You have been greeted from Python via Rust",
        );
    }
}

```

`src-tauri/src/commands/mod.rs`

```rust
mod api;
mod errors;
mod greet;

pub use greet::greet;

```

Make sure all Rust tests still pass.
