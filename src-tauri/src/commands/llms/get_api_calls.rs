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
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct GetApiCallsRequest {
        offset: i32,
    }

    async fn make_request_helper(
        args: &GetApiCallsRequest,
        side_effects: &SideEffectsHelpers,
    ) -> ZammResult<Vec<LightweightLlmCall>> {
        get_api_calls_helper(side_effects.db.as_ref().unwrap(), args.offset).await
    }

    impl_result_test_case!(
        GetApiCallsTestCase,
        get_api_calls,
        true,
        GetApiCallsRequest,
        Vec<LightweightLlmCall>
    );

    check_sample!(
        GetApiCallsTestCase,
        test_empty_list,
        "./api/sample-calls/get_api_calls-empty.yaml"
    );

    check_sample!(
        GetApiCallsTestCase,
        test_partial_list,
        "./api/sample-calls/get_api_calls-small.yaml"
    );

    check_sample!(
        GetApiCallsTestCase,
        test_full_list,
        "./api/sample-calls/get_api_calls-full.yaml"
    );

    check_sample!(
        GetApiCallsTestCase,
        test_offset,
        "./api/sample-calls/get_api_calls-offset.yaml"
    );

    check_sample!(
        GetApiCallsTestCase,
        test_empty_offset,
        "./api/sample-calls/get_api_calls-offset-empty.yaml"
    );

    check_sample!(
        GetApiCallsTestCase,
        test_unknown_provider_promptr,
        "./api/sample-calls/get_api_calls-unknown-provider-prompt.yaml"
    );
}
