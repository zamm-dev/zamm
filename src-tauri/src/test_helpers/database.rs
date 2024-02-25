use crate::setup::db::MIGRATIONS;
use crate::ZammDatabase;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use std::path::PathBuf;
use tokio::sync::Mutex;

pub fn setup_database(file: Option<&PathBuf>) -> SqliteConnection {
    let mut conn = match file {
        Some(file) => {
            SqliteConnection::establish(file.as_path().to_str().unwrap()).unwrap()
        }
        None => SqliteConnection::establish(":memory:").unwrap(),
    };

    conn.run_pending_migrations(MIGRATIONS).unwrap();
    conn
}

pub fn setup_zamm_db(file: Option<&PathBuf>) -> ZammDatabase {
    ZammDatabase(Mutex::new(Some(setup_database(file))))
}
