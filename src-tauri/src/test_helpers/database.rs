use crate::setup::db::MIGRATIONS;
use crate::ZammDatabase;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use tokio::sync::Mutex;

pub fn setup_database() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    conn
}

pub fn setup_zamm_db() -> ZammDatabase {
    ZammDatabase(Mutex::new(Some(setup_database())))
}
