use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{LightweightLlmCall, LlmCallRow};
use crate::schema::llm_calls;
use crate::ZammDatabase;
use anyhow::anyhow;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use specta::specta;
use tauri::State;

const PAGE_SIZE: i64 = 50;

async fn get_api_calls_helper(
    zamm_db: &ZammDatabase,
    offset: i32,
) -> ZammResult<Vec<LightweightLlmCall>> {
    let mut db = zamm_db.0.lock().await;
    let conn = db.as_mut().ok_or(anyhow!("Failed to lock database"))?;
    let result: Vec<LlmCallRow> = llm_calls::table
        .order(llm_calls::timestamp.desc())
        .offset(offset as i64)
        .limit(PAGE_SIZE)
        .load::<LlmCallRow>(conn)?;
    let calls: Vec<LightweightLlmCall> =
        result.into_iter().map(|row| row.into()).collect();
    Ok(calls)
}

#[tauri::command(async)]
#[specta]
pub async fn get_api_calls(
    database: State<'_, ZammDatabase>,
    offset: i32,
) -> ZammResult<Vec<LightweightLlmCall>> {
    get_api_calls_helper(&database, offset).await
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
    struct GetApiCallsRequest {
        offset: i32,
    }

    struct GetApiCallsTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<GetApiCallsRequest, ZammResult<Vec<LightweightLlmCall>>>
        for GetApiCallsTestCase
    {
        const EXPECTED_API_CALL: &'static str = "get_api_calls";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &GetApiCallsRequest,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<Vec<LightweightLlmCall>> {
            get_api_calls_helper(side_effects.db.as_ref().unwrap(), args.offset).await
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<Vec<LightweightLlmCall>>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: &GetApiCallsRequest,
            result: &ZammResult<Vec<LightweightLlmCall>>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<GetApiCallsRequest, Vec<LightweightLlmCall>>
        for GetApiCallsTestCase
    {
    }

    async fn check_get_api_calls_sample(test_fn_name: &'static str, file_prefix: &str) {
        let mut test_case = GetApiCallsTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_get_api_calls_empty() {
        check_get_api_calls_sample(
            function_name!(),
            "./api/sample-calls/get_api_calls-empty.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_api_calls_less_than_10() {
        check_get_api_calls_sample(
            function_name!(),
            "./api/sample-calls/get_api_calls-small.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_api_calls_full() {
        check_get_api_calls_sample(
            function_name!(),
            "./api/sample-calls/get_api_calls-full.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_api_calls_offset() {
        check_get_api_calls_sample(
            function_name!(),
            "./api/sample-calls/get_api_calls-offset.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_api_calls_offset_empty() {
        check_get_api_calls_sample(
            function_name!(),
            "./api/sample-calls/get_api_calls-offset-empty.yaml",
        )
        .await;
    }
}
