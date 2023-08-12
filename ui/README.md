# ZAMM

## Dev setup

Follow the instructions in [`tauri.md`](/zamm/resources/tutorials/setup/dev/tauri.md) to set up Tauri.

Then, to avoid the issue mentioned in [`indradb.md`](/zamm/resources/tutorials/libraries/indradb.md), install this:

```bash
$ sudo apt install libclang-dev
```

And to avoid the issue mentioned in [new Tauri project setup](/zamm/resources/tutorials/setup/tauri/new-tauri-project.md), install this:

```bash
$ sudo apt install fuse
```

### For new repos

Follow these instructions to set the project up:

- [Setting up a new Tauri project](/zamm/resources/tutorials/setup/tauri/new-tauri-project.md)
- [Setting up a Python sidecar](/zamm/resources/tutorials/setup/tauri/python-sidecar.md)

### For existing repos

If you already have a version of this project built, then enter into its directories and start building:

```bash
$ cd src-python
$ poetry shell
$ poetry install
$ make sidecar
```

and

```bash
$ pre-commit install
```

## Feature engineering

### Singleton Graph DB

First we set up a singleton graph database using Tauri's state management system. Import Mutex:

```rust
use std::sync::Mutex;
use oxigraph::store::StorageError;
```

Then define a GraphDB struct to store the new store in. Connect to "zamm.db" by default, but put it in a constant.

```rust
const DB_NAME: &str = "zamm.db";

struct GraphDB(Mutex<Store>);
```

Now add this new store to the state, from:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
```

to

```rust
fn main() -> Result<(), StorageError> {
    let store = Store::open(DB_NAME)?;

    tauri::Builder::default()
        .manage(GraphDB(Mutex::new(store)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
```

Check that you have gotten this to compile.

### Saving the graph DB to the user's data dir

Instead of leaving a mess in the arbitrary current directory where the command is run, or preventing the user from accessing the same database again, use [`directories`](/zamm/resources/tutorials/libraries/directories.md) to pick the user's data folder for app data storage.

- Make sure to use a constant for the database name.
- If there is no user home directory found, default to the current directory, and print out an error message to that effect.
- Make sure to print out the eventual graph database path

TODO: make database path a configurable commandline argument instead

Change

```rust
fn main() -> Result<(), StorageError> {
    let store = Store::open(DB_NAME)?;

    tauri::Builder::default()
        .manage(GraphDB(Mutex::new(store)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
```

to

```rust
fn main() -> Result<(), StorageError> {
    let db_path = if let Some(zamm_dirs) = ProjectDirs::from("dev", "zamm", "ZAMM") {
        zamm_dirs.data_dir().join(DB_NAME)
    } else {
        eprintln!("Cannot find user home directory, defaulting to current dir.");
        env::current_dir()?.as_path().join(DB_NAME)
    };
    let store = Store::open(db_path.as_path())?;
    let db_path_display = db_path.display();
    println!("Graph database opened at {db_path_display}");

    tauri::Builder::default()
        .manage(GraphDB(Mutex::new(store)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
```
