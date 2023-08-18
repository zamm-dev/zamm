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

We define it as such

```rust
const DB_NAME: &str = "zamm.sqlite3";

struct ZammDatabase(Mutex<Option<SqliteConnection>>);

...

fn main() {
    let possible_db = get_db();

    tauri::Builder::default()
        .manage(ZammDatabase(Mutex::new(possible_db)))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

where `get_db` is defined below.

Check that you have gotten this to compile.

### Saving the DB to the user's data dir

Instead of leaving a mess in the arbitrary current directory where the command is run, or preventing the user from accessing the same database again, use [`directories`](/zamm/resources/tutorials/libraries/directories.md) to pick the user's data folder for app data storage.

Requirements:

- Try to create the database in the user's data directory.
- If that fails for any reason, print out an error message to that effect, and default to creating the database in the current directory instead.
- Make sure to print out the eventual graph database path

Implementation details:

- Make sure to use a constant for the database name.

TODO: make database path a configurable commandline argument instead

This is a sample implementation of these requirements:

```rust
fn connect_to(db_path: PathBuf) -> Option<SqliteConnection> {
    let db_path_str = db_path.to_str().expect("Cannot convert DB path to str");
    match SqliteConnection::establish(db_path_str) {
        Ok(conn) => {
            println!("Connected to DB at {}", db_path_str);
            Some(conn)
        },
        Err(e) => {
            eprintln!("Failed to connect to DB: {}", e);
            None
        }
    }
}

/** Try to start SQLite database in user data dir. */
fn get_data_dir_db() -> Option<SqliteConnection> {
    if let Some(user_dirs) = ProjectDirs::from("dev", "zamm", "ZAMM") {
        let data_dir = user_dirs.data_dir();

        if !data_dir.exists() {
            match fs::create_dir_all(&data_dir) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Failed to create data directory: {}", e);
                    return None;
                }
            }
        }

        connect_to(data_dir.join(DB_NAME))
    } else {
        eprintln!("Cannot find user home directory.");
        None
    }
}

fn get_db() -> Option<SqliteConnection> {
    get_data_dir_db().or_else(|| {
        eprintln!("Unable to create DB in user data dir, defaulting to current dir instead.");
        connect_to(env::current_dir().expect("Failed to get current directory").join(DB_NAME))
    })
}
```
