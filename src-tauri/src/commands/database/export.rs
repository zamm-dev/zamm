use crate::commands::database::metadata::DatabaseCounts;
use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{LlmCallFollowUp, LlmCallRow, LlmCallVariant};
use crate::models::{ApiKey, DatabaseContents, LlmCallData};
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

pub async fn get_database_contents(
    zamm_db: &ZammDatabase,
    save_version: bool,
) -> ZammResult<DatabaseContents> {
    let db_mutex: &mut MutexGuard<'_, Option<SqliteConnection>> =
        &mut zamm_db.0.lock().await;
    let db = db_mutex.as_mut().ok_or(anyhow!("Error getting db"))?;

    let zamm_version = if save_version {
        Some(env!("CARGO_PKG_VERSION").to_string())
    } else {
        None
    };
    let api_keys = api_keys::table.load::<ApiKey>(db)?;
    let llm_calls_instances = llm_calls::table.load::<LlmCallRow>(db)?;
    let follow_ups = llm_call_follow_ups::table.load::<LlmCallFollowUp>(db)?;
    let variants = llm_call_variants::table.load::<LlmCallVariant>(db)?;

    Ok(DatabaseContents {
        zamm_version,
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
    save_version: bool,
) -> ZammResult<DatabaseCounts> {
    let file_path_buf = PathBuf::from(file_path);
    let file_path_abs = file_path_buf.absolutize()?;
    let db_contents = get_database_contents(zamm_db, save_version).await?;
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
    Ok(DatabaseCounts {
        num_api_keys: db_contents.api_keys.len() as i32,
        num_llm_calls: db_contents.llm_calls.instances.len() as i32,
    })
}

async fn export_db_helper(
    zamm_db: &ZammDatabase,
    path: &str,
) -> ZammResult<DatabaseCounts> {
    write_database_contents(zamm_db, path, true).await
}

#[tauri::command(async)]
#[specta]
pub async fn export_db(
    database: State<'_, ZammDatabase>,
    path: String,
) -> ZammResult<DatabaseCounts> {
    export_db_helper(&database, &path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_call::SampleCall;
    use crate::test_helpers::api_testing::standard_test_subdir;
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use serde::{Deserialize, Serialize};
    use stdext::function_name;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ExportDbRequest {
        path: String,
    }

    struct ExportDbTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<ExportDbRequest, ZammResult<DatabaseCounts>>
        for ExportDbTestCase
    {
        const EXPECTED_API_CALL: &'static str = "export_db";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &ExportDbRequest,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<DatabaseCounts> {
            export_db_helper(side_effects.db.as_ref().unwrap(), &args.path).await
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<DatabaseCounts>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: &ExportDbRequest,
            result: &ZammResult<DatabaseCounts>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<ExportDbRequest, DatabaseCounts> for ExportDbTestCase {}

    async fn check_get_api_call_sample(test_fn_name: &'static str, file_prefix: &str) {
        let mut test_case = ExportDbTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_export_db_populated() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/export_db-populated.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_export_db_api_key() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/export_db-api-key.yaml",
        )
        .await;
    }
}
