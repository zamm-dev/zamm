use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{
    ChatMessage, EntityId, LlmCall, LlmCallLeftJoinResult, NewLlmCallFollowUp,
    NewLlmCallRow, NewLlmCallVariant,
};
use crate::models::{ApiKey, NewApiKey};
use crate::schema::{api_keys, llm_call_follow_ups, llm_call_variants, llm_calls};
use crate::views::{llm_call_named_follow_ups, llm_call_named_variants};
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

    pub fn insertable_call_follow_ups(&self) -> Vec<NewLlmCallFollowUp> {
        self.llm_calls
            .iter()
            .filter_map(|k| k.as_follow_up_row())
            .collect()
    }

    pub fn insertable_call_variants(&self) -> Vec<NewLlmCallVariant> {
        self.llm_calls
            .iter()
            .flat_map(|k| k.as_variant_rows())
            .collect()
    }
}

pub async fn get_database_contents(
    zamm_db: &ZammDatabase,
) -> ZammResult<DatabaseContents> {
    let db_mutex: &mut MutexGuard<'_, Option<SqliteConnection>> =
        &mut zamm_db.0.lock().await;
    let db = db_mutex.as_mut().ok_or(anyhow!("Error getting db"))?;
    let api_keys = api_keys::table.load::<ApiKey>(db)?;
    let llm_call_left_joins = llm_calls::table
        .left_join(
            llm_call_named_follow_ups::dsl::llm_call_named_follow_ups
                .on(llm_calls::id.eq(llm_call_named_follow_ups::next_call_id)),
        )
        .left_join(
            llm_call_named_variants::dsl::llm_call_named_variants
                .on(llm_calls::id.eq(llm_call_named_variants::variant_id)),
        )
        .select((
            llm_calls::all_columns,
            llm_call_named_follow_ups::previous_call_id.nullable(),
            llm_call_named_follow_ups::previous_call_completion.nullable(),
            llm_call_named_variants::canonical_id.nullable(),
            llm_call_named_variants::canonical_completion.nullable(),
        ))
        .get_results::<LlmCallLeftJoinResult>(db)?;
    let llm_calls_result: ZammResult<Vec<LlmCall>> = llm_call_left_joins
        .into_iter()
        .map(|lf| {
            let llm_call_id = lf.0.id.clone();
            let next_calls_result: Vec<(EntityId, ChatMessage)> =
                llm_call_named_follow_ups::table
                    .select((
                        llm_call_named_follow_ups::next_call_id,
                        llm_call_named_follow_ups::next_call_completion,
                    ))
                    .filter(
                        llm_call_named_follow_ups::previous_call_id.eq(&llm_call_id),
                    )
                    .load::<(EntityId, ChatMessage)>(db)?;
            let canonical_id = lf.3.clone().unwrap_or(llm_call_id);
            let variants_result: Vec<(EntityId, ChatMessage)> =
                llm_call_named_variants::table
                    .select((
                        llm_call_named_variants::variant_id,
                        llm_call_named_variants::variant_completion,
                    ))
                    .filter(llm_call_named_variants::canonical_id.eq(canonical_id))
                    .load::<(EntityId, ChatMessage)>(db)?;
            Ok((lf, next_calls_result, variants_result).into())
        })
        .collect();
    Ok(DatabaseContents {
        api_keys,
        llm_calls: llm_calls_result?,
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
        diesel::insert_into(llm_call_follow_ups::table)
            .values(&db_contents.insertable_call_follow_ups())
            .execute(conn)?;
        diesel::insert_into(llm_call_variants::table)
            .values(&db_contents.insertable_call_variants())
            .execute(conn)?;
        Ok(())
    })?;
    Ok(())
}

pub fn dump_sqlite_database(db_path: &PathBuf, dump_path: &PathBuf) {
    let dump_output = std::process::Command::new("sqlite3")
        .arg(db_path)
        // avoid the inserts into __diesel_schema_migrations
        .arg(".dump api_keys llm_calls llm_call_follow_ups llm_call_variants")
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
