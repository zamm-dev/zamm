# Refactoring a Rust file into its own directory

Say you have a file such as `src-tauri/src/setup.rs` that you want to turn into a directory:

```rust
use diesel::sqlite::SqliteConnection;
use directories::ProjectDirs;

use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::fs;
use std::path::PathBuf;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

const DB_NAME: &str = "zamm.sqlite3";

fn connect_to(db_path: PathBuf) -> Option<SqliteConnection> {
    let db_path_str = db_path.to_str().expect("Cannot convert DB path to str");
    match SqliteConnection::establish(db_path_str) {
        Ok(conn) => {
            println!("Connected to DB at {}", db_path_str);
            Some(conn)
        }
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
            match fs::create_dir_all(data_dir) {
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

pub fn get_db() -> Option<SqliteConnection> {
    let mut possible_connection = get_data_dir_db().or_else(|| {
        eprintln!(
            "Unable to create DB in user data dir, defaulting to current dir instead."
        );
        connect_to(
            env::current_dir()
                .expect("Failed to get current directory")
                .join(DB_NAME),
        )
    });
    if let Some(connection) = possible_connection.as_mut() {
        match connection.run_pending_migrations(MIGRATIONS) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to run migrations: {}", e);
                return None;
            }
        }
    }
    possible_connection
}

```

First, move that file to `src-tauri/src/setup/mod.rs`. Then identify what functionality you want to export, what functionality is common between different files in the module, and what functionality is self-contained. In this case, everything involving the database setup is self-contained, and we only want the module to export the `db` submodule and the `get_db` function. We therefore split `mod.rs` into:

```rust
pub mod db;

pub use db::get_db;
```

and `src-tauri/src/setup/db.rs`:

```rust
use diesel::sqlite::SqliteConnection;
use directories::ProjectDirs;

use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::fs;
use std::path::PathBuf;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

const DB_NAME: &str = "zamm.sqlite3";

fn connect_to(db_path: PathBuf) -> Option<SqliteConnection> {
    let db_path_str = db_path.to_str().expect("Cannot convert DB path to str");
    match SqliteConnection::establish(db_path_str) {
        Ok(conn) => {
            println!("Connected to DB at {}", db_path_str);
            Some(conn)
        }
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
            match fs::create_dir_all(data_dir) {
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

pub fn get_db() -> Option<SqliteConnection> {
    let mut possible_connection = get_data_dir_db().or_else(|| {
        eprintln!(
            "Unable to create DB in user data dir, defaulting to current dir instead."
        );
        connect_to(
            env::current_dir()
                .expect("Failed to get current directory")
                .join(DB_NAME),
        )
    });
    if let Some(connection) = possible_connection.as_mut() {
        match connection.run_pending_migrations(MIGRATIONS) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to run migrations: {}", e);
                return None;
            }
        }
    }
    possible_connection
}
```

Clean up any imports that have changed. As it happens, in this example there is one other import in `src-tauri/src/models.rs`, which we have changed to the new migrations import path:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::db::MIGRATIONS;

    ...
}
```

Now we can set up other functionality as needed. For exapmle, we can add a `src-tauri/src/setup/api_keys.rs`:

```rust
use std::env;

pub enum Source {
    Environment,
}

pub struct ApiKey {
    pub value: String,
    pub source: Source,
}

pub struct ApiKeys {
    pub openai: Option<ApiKey>,
}

pub fn get_api_keys() -> ApiKeys {
    let mut api_keys = ApiKeys {
        openai: None,
    };
    if let Ok(openai_api_key) = env::var("OPENAI_API_KEY") {
        api_keys.openai = Some(ApiKey {
            value: openai_api_key,
            source: Source::Environment,
        });
    }
    api_keys
}

```

and we add this new file to `src-tauri/src/setup/mod.rs`:

```rust
pub mod api_keys;
pub mod db;

pub use api_keys::get_api_keys;
pub use db::get_db;

```

and finally to `src-tauri/src/main.rs`:

```rust
use setup::api_keys::{get_api_keys, ApiKeys};

...

fn main() {
    ...

    tauri::Builder::default()
        ...
        .manage(ZammApiKeys(Mutex::new(get_api_keys())))
        ...
        .expect("error while running tauri application");
}
```
