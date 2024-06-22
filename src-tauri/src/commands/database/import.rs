use specta::specta;

use tauri::State;

use crate::commands::errors::ZammResult;
use crate::models::database_contents::read_database_contents;
use crate::ZammDatabase;

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
    async fn test_import_db_conflicting() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/import_db-conflicting.yaml",
        )
        .await;
    }
}
