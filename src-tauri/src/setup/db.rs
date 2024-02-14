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
