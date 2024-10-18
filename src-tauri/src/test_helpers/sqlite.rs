use crate::ZammDatabase;
use diesel::connection::SimpleConnection;
use std::fs;
use std::path::PathBuf;

pub async fn load_sqlite_database(zamm_db: &ZammDatabase, dump_path: &PathBuf) {
    let db = &mut zamm_db.0.lock().await;
    let conn = db.as_mut().unwrap();
    let dump = fs::read_to_string(dump_path).expect("Error reading dump file");
    conn.batch_execute(&dump)
        .expect("Error loading dump into database");
}

pub fn dump_sqlite_database(db_path: &PathBuf, dump_path: &PathBuf) {
    let dump_output = std::process::Command::new("sqlite3")
        .arg(db_path)
        // avoid the inserts into __diesel_schema_migrations
        .arg(
            ".dump api_keys llm_calls llm_call_follow_ups llm_call_variants asciicasts",
        )
        .output()
        .expect("Error running sqlite3 .dump command");
    // filter output by lines starting with "INSERT"
    let inserts = String::from_utf8_lossy(&dump_output.stdout)
        .lines()
        .filter(|line| line.starts_with("INSERT"))
        .collect::<Vec<&str>>()
        .join("\n");
    fs::write(dump_path, inserts).expect("Error writing dump file");
}
