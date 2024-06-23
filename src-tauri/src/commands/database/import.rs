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
    Ok(())
}

async fn import_db_helper(zamm_db: &ZammDatabase, path: &str) -> ZammResult<()> {
    read_database_contents(zamm_db, path).await
}

#[tauri::command(async)]
#[specta]
pub async fn import_db(
    database: State<'_, ZammDatabase>,
    path: &str,
) -> ZammResult<()> {
    import_db_helper(&database, path).await
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
    struct ImportDbRequest {
        path: String,
    }

    struct ImportDbTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<ImportDbRequest, ZammResult<()>> for ImportDbTestCase {
        const EXPECTED_API_CALL: &'static str = "import_db";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<ImportDbRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<()> {
            import_db_helper(
                side_effects.db.as_ref().unwrap(),
                &args.as_ref().unwrap().path,
            )
            .await
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<()>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&ImportDbRequest>,
            result: &ZammResult<()>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<ImportDbRequest, ()> for ImportDbTestCase {}

    async fn check_get_api_call_sample(test_fn_name: &'static str, file_prefix: &str) {
        let mut test_case = ImportDbTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_import_db_initially_empty() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/import_db-initially-empty.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_import_db_api_key() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/import_db-api-key.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_import_db_conflicting_llm_call() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/import_db-conflicting-llm-call.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_import_db_conflicting_api_key() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/import_db-conflicting-api-key.yaml",
        )
        .await;
    }
}
