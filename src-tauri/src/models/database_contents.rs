use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{
    LlmCallFollowUp, LlmCallRow, LlmCallVariant, NewLlmCallFollowUp, NewLlmCallRow,
    NewLlmCallVariant,
};
use crate::models::{ApiKey, NewApiKey};
use crate::schema::{api_keys, llm_call_follow_ups, llm_call_variants, llm_calls};
use crate::ZammDatabase;
use anyhow::anyhow;

use diesel::prelude::*;
use path_absolutize::Absolutize;
use std::fs;
use std::path::PathBuf;
use tokio::sync::MutexGuard;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LlmCallData {
    instances: Vec<LlmCallRow>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    follow_ups: Vec<LlmCallFollowUp>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    variants: Vec<LlmCallVariant>,
}

impl LlmCallData {
    pub fn is_default(&self) -> bool {
        self.instances.is_empty()
            && self.follow_ups.is_empty()
            && self.variants.is_empty()
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DatabaseContents {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    api_keys: Vec<ApiKey>,
    #[serde(skip_serializing_if = "LlmCallData::is_default", default)]
    llm_calls: LlmCallData,
}

impl DatabaseContents {
    pub fn insertable_api_keys(&self) -> Vec<NewApiKey> {
        self.api_keys.iter().map(|k| k.as_insertable()).collect()
    }

    pub fn insertable_llm_calls(&self) -> Vec<NewLlmCallRow> {
        self.llm_calls
            .instances
            .iter()
            .map(|k| k.as_insertable())
            .collect()
    }

    pub fn insertable_call_follow_ups(&self) -> Vec<NewLlmCallFollowUp> {
        self.llm_calls
            .follow_ups
            .iter()
            .map(|k| k.as_insertable())
            .collect()
    }

    pub fn insertable_call_variants(&self) -> Vec<NewLlmCallVariant> {
        self.llm_calls
            .variants
            .iter()
            .map(|k| k.as_insertable())
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
    let llm_calls_instances = llm_calls::table.load::<LlmCallRow>(db)?;
    let follow_ups = llm_call_follow_ups::table.load::<LlmCallFollowUp>(db)?;
    let variants = llm_call_variants::table.load::<LlmCallVariant>(db)?;

    Ok(DatabaseContents {
        api_keys,
        llm_calls: LlmCallData {
            instances: llm_calls_instances,
            follow_ups,
            variants,
        },
    })
}

pub async fn write_database_contents(
    zamm_db: &ZammDatabase,
    file_path: &str,
) -> ZammResult<()> {
    let file_path_buf = PathBuf::from(file_path);
    let file_path_abs = file_path_buf.absolutize()?;
    let db_contents = get_database_contents(zamm_db).await?;
    let serialized = serde_yaml::to_string(&db_contents)?;
    if let Some(parent) = file_path_abs.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            anyhow!(
                "Error creating parent directory {}: {}",
                parent.display(),
                e
            )
        })?;
    }
    fs::write(&file_path_abs, serialized).map_err(|e| {
        anyhow!("Error exporting to {}: {}", &file_path_abs.display(), e)
    })?;
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

    let new_llm_calls: Vec<NewLlmCallRow> = db_contents
        .insertable_llm_calls()
        .into_iter()
        .filter(|call| {
            llm_calls::table
                .filter(llm_calls::id.eq(&call.id))
                .count()
                .get_result::<i64>(db)
                .unwrap_or(0)
                == 0
        })
        .collect();
    let new_llm_call_ids = new_llm_calls.iter().map(|call| call.id).collect::<Vec<_>>();
    let new_llm_call_follow_ups: Vec<NewLlmCallFollowUp> = db_contents
        .insertable_call_follow_ups()
        .into_iter()
        .filter(|follow_up| {
            new_llm_call_ids.contains(&follow_up.previous_call_id)
                || new_llm_call_ids.contains(&follow_up.next_call_id)
        })
        .collect();
    let new_llm_call_variants: Vec<NewLlmCallVariant> = db_contents
        .insertable_call_variants()
        .into_iter()
        .filter(|variant| {
            new_llm_call_ids.contains(&variant.canonical_id)
                || new_llm_call_ids.contains(&variant.variant_id)
        })
        .collect();

    db.transaction::<(), diesel::result::Error, _>(|conn| {
        diesel::insert_into(api_keys::table)
            .values(&db_contents.insertable_api_keys())
            .execute(conn)?;
        diesel::insert_into(llm_calls::table)
            .values(&new_llm_calls)
            .execute(conn)?;
        diesel::insert_into(llm_call_follow_ups::table)
            .values(&new_llm_call_follow_ups)
            .execute(conn)?;
        diesel::insert_into(llm_call_variants::table)
            .values(&new_llm_call_variants)
            .execute(conn)?;
        Ok(())
    })?;
    Ok(())
}
