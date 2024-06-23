use specta::specta;

use tauri::State;

use crate::commands::errors::ZammResult;
use crate::models::database_contents::write_database_contents;
use crate::ZammDatabase;

async fn export_db_helper(zamm_db: &ZammDatabase, path: &str) -> ZammResult<()> {
    write_database_contents(zamm_db, path, true).await
}

#[tauri::command(async)]
#[specta]
pub async fn export_db(
    database: State<'_, ZammDatabase>,
    path: String,
) -> ZammResult<()> {
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

    impl SampleCallTestCase<ExportDbRequest, ZammResult<()>> for ExportDbTestCase {
        const EXPECTED_API_CALL: &'static str = "export_db";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<ExportDbRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<()> {
            export_db_helper(
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
            args: Option<&ExportDbRequest>,
            result: &ZammResult<()>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<ExportDbRequest, ()> for ExportDbTestCase {}

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
