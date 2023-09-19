# Using mockall

First install it:

```bash
$ cargo add mockall
```

Then use it to mock your dependencies. You may need to refactor your code to make it more testable. For example, if you have a function that uses `Command::new_sidecar`, which you want to mock:

```rust
#[tauri::command]
#[specta]
fn greet(name: &str) -> String {
    let expected_binary_path = relative_command_path("zamm-python".to_string())
        .expect("Failed to get expected binary path");
    let (mut rx, mut _child) = Command::new_sidecar("zamm-python")
        .expect("failed to create `zamm-python` binary command")
        .args(vec![name])
        .spawn()
        .unwrap_or_else(|err| {
            panic!(
                "Failed to spawn sidecar at {}: {}",
                expected_binary_path, err
            )
        });

    // https://stackoverflow.com/a/52521592
    let result = executor::block_on(tauri::async_runtime::spawn(async move {
        let mut last_line = "No output".to_string();
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                last_line = format!("{} via Rust", line);
            }
        }
        last_line
    }));
    result.unwrap_or("Failed to get output".to_string())
}
```

change it to:

```rust
use anyhow::Result;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Failed to spawn sidecar at {expected_path}: {tauri_error}")]
    SidecarSpawnError {
        expected_path: String, tauri_error: tauri::api::Error,}
}

trait SidecarExecutor {
    fn execute<I, S>(command: &str, args: I) -> Result<String>
    where
      I: IntoIterator<Item = S>,
      S: AsRef<str>;
}

struct SidecarExecutorImpl;

impl SidecarExecutor for SidecarExecutorImpl {
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
}

#[tauri::command]
#[specta]
fn greet(name: &str) -> String {
    let result = SidecarExecutorImpl::execute("zamm-python", vec![name]).unwrap();
    format!("{result} via Rust")
}
```

Note the use of [`thiserror`](./thiserror.md) and [`anyhow`](./anyhow.md)

Now we'll mock the trait:

```rust
use mockall::automock;

#[automock]
trait SidecarExecutor {
    #[allow(clippy::needless_lifetimes)]
    fn execute<'a>(&self, command: &str, args: &[&'a str]) -> Result<String>;
}

struct SidecarExecutorImpl;

impl SidecarExecutor for SidecarExecutorImpl {
    fn execute(&self, command: &str, args: &[&str]) -> Result<String> {
        ...
    }
}

fn greet_helper<T: SidecarExecutor>(t: &T, name: &str) -> String {
    let result = t.execute("zamm-python", &[name]).unwrap();
    format!("{result} via Rust")
}

#[tauri::command]
#[specta]
fn greet(name: &str) -> String {
    greet_helper(&SidecarExecutorImpl {}, name)
}

...

#[cfg(test)]
mod tests {
    use super::{greet_helper, MockSidecarExecutor};

    #[test]
    fn test_greet_name() {
        let mut mock = MockSidecarExecutor::new();
        mock.expect_execute()
            .withf(|cmd, args| {
                assert_eq!(cmd, "zamm-python");
                assert_eq!(args, &vec!["Test"]);
                true
            })
            .returning(|_, _| {
                Ok("Hello, Test! You have been greeted from Python".to_string())
            });

        let result = greet_helper(&mock, "Test");
        assert_eq!(
            result,
            "Hello, Test! You have been greeted from Python via Rust"
        );
    }
}
```

Note that we changed the signature of `execute` because it results in the error

```
error[E0310]: the parameter type `S` may not live long enough
    --> src/main.rs:44:1
     |
44   | #[automock]
     | ^^^^^^^^^^^ ...so that the type `Expectations<I, S>` will meet its required lifetime bounds...
     |
note: ...that is required by this bound
    --> /root/.asdf/installs/rust/1.71.1/registry/src/index.crates.io-6f17d22bba15001f/mockall-0.11.4/src/lib.rs:1354:29
     |
1354 | pub trait AnyExpectations : Any + Send + Sync {}
     |                             ^^^
     = note: this error originates in the attribute macro `automock` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider adding an explicit lifetime bound...
     |
49   |       S: AsRef<str> + 'static;
     |                     +++++++++
...
```

and it appears the usage of generics this way may not be supported by `mockall`.

We also make sure to use `&[&'a str]` instead of `&Vec<&'a str>` because it results in the warning

```
warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
  --> src/main.rs:49:48
   |
49 |     fn execute<'a>(&self, command: &str, args: &Vec<&'a str>) -> Result<String>;
   |                                                ^^^^^^^^^^^^^ help: change this to: `&[&'a str]`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
   = note: `-D clippy::ptr-arg` implied by `-D warnings`
```

On the other hand, this error should be ignored:

```rust
warning: the following explicit lifetimes could be elided: 'a
  --> src/main.rs:47:5
   |
47 |     fn execute<'a>(&self, command: &str, args: &[&'a str]) -> Result<String>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_lifetimes
   = note: `-D clippy::needless-lifetimes` implied by `-D warnings`
help: elide the lifetimes
   |
47 -     fn execute<'a>(&self, command: &str, args: &[&'a str]) -> Result<String>;
47 +     fn execute(&self, command: &str, args: &[&str]) -> Result<String>;
   |
```

Following the suggestion as such:

```rust
#[automock]
trait SidecarExecutor {
    fn execute(&self, command: &str, args: &[&str]) -> Result<String>;
}
```

results in the compilation error

```
error[E0106]: missing lifetime specifier
  --> src/main.rs:47:46
   |
47 |     fn execute(&self, command: &str, args: &[&str]) -> Result<String>;
   |                                              ^ expected named lifetime parameter
   |
   = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider making the bound lifetime-generic with a new `'a` lifetime
   |
45 ~ for<'a> #[automock]
46 | trait SidecarExecutor {
47 ~     fn execute(&self, command: &str, args: &[&'a str]) -> Result<String>;
   |
help: consider introducing a named lifetime parameter
   |
45 ~ #[automock]<'a>
46 | trait SidecarExecutor {
47 ~     fn execute(&self, command: &str, args: &[&'a str]) -> Result<String>;
   |

error[E0637]: `&` without an explicit lifetime name cannot be used here
  --> src/main.rs:47:46
   |
47 |     fn execute(&self, command: &str, args: &[&str]) -> Result<String>;
   |                                              ^ explicit lifetime name needed here
   |
help: consider introducing a higher-ranked lifetime here with `for<'a>`
  --> src/main.rs:45:1
   |
45 | #[automock]
   | ^
   = note: this error originates in the attribute macro `automock` (in Nightly builds, run with -Z macro-backtrace for more info)
```

Neither of the proposed fixes work:

```
error: expected item, found keyword `for`
  --> /root/zamm/src-tauri/src/main.rs:45:1
   |
45 | for<'a> #[automock]
   | ^^^ expected item
```

and

```
error: expected identifier, found `<`
  --> /root/zamm/src-tauri/src/main.rs:45:12
   |
45 | #[automock]<'a>
   |            ^ expected identifier
```

Finally, note that we do explicit assertions in `withf` because if we instead did

```rust
        mock.expect_execute()
            .withf(|cmd, args| {
                cmd == "zamm-python" && args == &vec!["Test"]
            })
            .returning(|_, _| {
                Ok("Hello, Test! You have been greeted from Python".to_string())
            });
```

and then failed the test

```rust
        let result = greet_helper(&mock, "Tests");
```

the error message is an unhelpful

```
---- tests::test_greet_name stdout ----
thread 'tests::test_greet_name' panicked at 'MockSidecarExecutor::execute(?, ?): No matching expectation found', src/main.rs:47:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Versus an explicit assert:

```
---- tests::test_greet_name stdout ----
thread 'tests::test_greet_name' panicked at 'assertion failed: `(left == right)`
  left: `["Tests"]`,
 right: `["Test"]`', src/main.rs:121:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Cleaning up

Refactor the code to move all command-related testing code into its own file. See commit `61d287a`. `main.rs` should look like this in the end:

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diesel::sqlite::SqliteConnection;

#[cfg(debug_assertions)]
use specta::collect_types;

#[cfg(debug_assertions)]
use tauri_specta::ts;

use std::env;

use std::sync::Mutex;
mod commands;
mod models;
mod schema;
mod setup;
use commands::greet;

struct ZammDatabase(Mutex<Option<SqliteConnection>>);

fn main() {
    #[cfg(debug_assertions)]
    ts::export(collect_types![greet], "../src-svelte/src/lib/bindings.ts").unwrap();

    let possible_db = setup::get_db();

    tauri::Builder::default()
        .manage(ZammDatabase(Mutex::new(possible_db)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
