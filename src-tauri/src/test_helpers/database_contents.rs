use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{LlmCall, LlmCallRow, NewLlmCallRow};
use crate::models::{ApiKey, NewApiKey};
use crate::schema::{api_keys, llm_calls};
use crate::ZammDatabase;
use anyhow::anyhow;
use diesel::prelude::*;
use path_absolutize::Absolutize;
use std::fs;
use std::path::PathBuf;
use tokio::sync::MutexGuard;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DatabaseContents {
    api_keys: Vec<ApiKey>,
    llm_calls: Vec<LlmCall>,
}

impl DatabaseContents {
    pub fn insertable_api_keys(&self) -> Vec<NewApiKey> {
        self.api_keys.iter().map(|k| k.as_insertable()).collect()
    }

    pub fn insertable_llm_calls(&self) -> Vec<NewLlmCallRow> {
        self.llm_calls.iter().map(|k| k.as_sql_row()).collect()
    }
}

pub async fn get_database_contents(
    zamm_db: &ZammDatabase,
) -> ZammResult<DatabaseContents> {
    let db_mutex: &mut MutexGuard<'_, Option<SqliteConnection>> =
        &mut zamm_db.0.lock().await;
    let db = db_mutex.as_mut().ok_or(anyhow!("Error getting db"))?;
    let api_keys = api_keys::table.load::<ApiKey>(db)?;
    let llm_call_rows = llm_calls::table.load::<LlmCallRow>(db)?;
    let llm_calls: Vec<LlmCall> = llm_call_rows.into_iter().map(|r| r.into()).collect();
    Ok(DatabaseContents {
        api_keys,
        llm_calls,
    })
}

pub async fn write_database_contents(
    zamm_db: &ZammDatabase,
    file_path: &PathBuf,
) -> ZammResult<()> {
    let db_contents = get_database_contents(zamm_db).await?;
    let serialized = serde_yaml::to_string(&db_contents)?;
    fs::write(file_path, serialized)?;
    Ok(())
}

pub async fn read_database_contents(
    zamm_db: &ZammDatabase,
    file_path: &str,
) -> ZammResult<()> {
    let db_mutex: &mut MutexGuard<'_, Option<SqliteConnection>> =
        &mut zamm_db.0.lock().await;
    let db = db_mutex.as_mut().ok_or(anyhow!("Error getting db"))?;

    let file_path_buf = PathBuf::from(file_path);
    let file_path_abs = file_path_buf.absolutize()?;
    let serialized = fs::read_to_string(&file_path_abs).map_err(|e| {
        anyhow!("Error reading file at {}: {}", &file_path_abs.display(), e)
    })?;
    let db_contents: DatabaseContents = serde_yaml::from_str(&serialized)?;
    db.transaction::<(), diesel::result::Error, _>(|conn| {
        diesel::insert_into(api_keys::table)
            .values(&db_contents.insertable_api_keys())
            .execute(conn)?;
        diesel::insert_into(llm_calls::table)
            .values(&db_contents.insertable_llm_calls())
            .execute(conn)?;
        Ok(())
    })?;
    Ok(())
}

pub fn dump_sqlite_database(db_path: &PathBuf, dump_path: &PathBuf) {
    let dump_output = std::process::Command::new("sqlite3")
        .arg(db_path)
        // avoid the inserts into __diesel_schema_migrations
        .arg(".dump api_keys llm_calls")
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
