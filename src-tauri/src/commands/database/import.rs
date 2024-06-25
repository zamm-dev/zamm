use crate::commands::database::metadata::DatabaseCounts;
use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{NewLlmCallFollowUp, NewLlmCallRow, NewLlmCallVariant};
use crate::models::{DatabaseContents, NewApiKey};
use crate::schema::{api_keys, llm_call_follow_ups, llm_call_variants, llm_calls};
use crate::ZammDatabase;
use anyhow::anyhow;
use diesel::prelude::*;
use path_absolutize::Absolutize;
use specta::specta;
use std::fs;
use std::path::PathBuf;
use tauri::State;
use tokio::sync::MutexGuard;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DatabaseImportCounts {
    pub imported: DatabaseCounts,
    pub ignored: DatabaseCounts,
}

pub async fn read_database_contents(
    zamm_db: &ZammDatabase,
    file_path: &str,
) -> ZammResult<DatabaseImportCounts> {
    let db_mutex: &mut MutexGuard<'_, Option<SqliteConnection>> =
        &mut zamm_db.0.lock().await;
    let db = db_mutex.as_mut().ok_or(anyhow!("Error getting db"))?;

    let file_path_buf = PathBuf::from(file_path);
    let file_path_abs = file_path_buf.absolutize()?;
    let serialized = fs::read_to_string(&file_path_abs).map_err(|e| {
        anyhow!("Error reading file at {}: {}", &file_path_abs.display(), e)
    })?;
    let db_contents: DatabaseContents = serde_yaml::from_str(&serialized)?;

    let new_api_keys: Vec<NewApiKey> = db_contents
        .insertable_api_keys()
        .into_iter()
        .filter(|key| {
            api_keys::table
                .filter(api_keys::service.eq(&key.service))
                .count()
                .get_result::<i64>(db)
                .unwrap_or(0)
                == 0
        })
        .collect();
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
            .values(&new_api_keys)
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
    Ok(DatabaseImportCounts {
        imported: DatabaseCounts {
            num_api_keys: new_api_keys.len() as i32,
            num_llm_calls: new_llm_calls.len() as i32,
        },
        ignored: DatabaseCounts {
            num_api_keys: (db_contents.api_keys.len() - new_api_keys.len()) as i32,
            num_llm_calls: (db_contents.llm_calls.instances.len() - new_llm_calls.len())
                as i32,
        },
    })
}

async fn import_db_helper(
    zamm_db: &ZammDatabase,
    path: &str,
) -> ZammResult<DatabaseImportCounts> {
    read_database_contents(zamm_db, path).await
}

#[tauri::command(async)]
#[specta]
pub async fn import_db(
    database: State<'_, ZammDatabase>,
    path: &str,
) -> ZammResult<DatabaseImportCounts> {
    import_db_helper(&database, path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ImportDbRequest {
        path: String,
    }

    async fn make_request_helper(
        args: &ImportDbRequest,
        side_effects: &SideEffectsHelpers,
    ) -> ZammResult<DatabaseImportCounts> {
        import_db_helper(side_effects.db.as_ref().unwrap(), &args.path).await
    }

    impl_result_test_case!(
        ImportDbTestCase,
        import_db,
        true,
        ImportDbRequest,
        DatabaseImportCounts
    );

    check_sample!(
        ImportDbTestCase,
        test_initially_empty,
        "./api/sample-calls/import_db-initially-empty.yaml"
    );

    check_sample!(
        ImportDbTestCase,
        test_api_key,
        "./api/sample-calls/import_db-api-key.yaml"
    );

    check_sample!(
        ImportDbTestCase,
        test_conflicting_llm_call,
        "./api/sample-calls/import_db-conflicting-llm-call.yaml"
    );

    check_sample!(
        ImportDbTestCase,
        test_conflicting_api_key,
        "./api/sample-calls/import_db-conflicting-api-key.yaml"
    );
}
