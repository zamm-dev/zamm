use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{ChatMessage, EntityId, LlmCall, LlmCallLeftJoinResult};
use crate::schema::llm_calls;
use crate::views::llm_call_named_continuations;
use crate::ZammDatabase;
use anyhow::anyhow;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use specta::specta;
use tauri::State;
use uuid::Uuid;

async fn get_api_call_helper(
    zamm_db: &ZammDatabase,
    api_call_id: &str,
) -> ZammResult<LlmCall> {
    let parsed_uuid = EntityId {
        uuid: Uuid::parse_str(api_call_id)?,
    };
    let mut db = zamm_db.0.lock().await;
    let conn = db.as_mut().ok_or(anyhow!("Failed to lock database"))?;

    let previous_call_result: LlmCallLeftJoinResult = llm_calls::table
        .left_join(
            llm_call_named_continuations::dsl::llm_call_named_continuations
                .on(llm_calls::id.eq(llm_call_named_continuations::next_call_id)),
        )
        .select((
            llm_calls::all_columns,
            llm_call_named_continuations::previous_call_id.nullable(),
            llm_call_named_continuations::previous_call_completion.nullable(),
        ))
        .filter(llm_calls::id.eq(&parsed_uuid))
        .first::<LlmCallLeftJoinResult>(conn)?;
    let next_calls_result = llm_call_named_continuations::table
        .select((
            llm_call_named_continuations::next_call_id,
            llm_call_named_continuations::next_call_completion,
        ))
        .filter(llm_call_named_continuations::previous_call_id.eq(parsed_uuid))
        .load::<(EntityId, ChatMessage)>(conn)?;
    Ok((previous_call_result, next_calls_result).into())
}

#[tauri::command(async)]
#[specta]
pub async fn get_api_call(
    database: State<'_, ZammDatabase>,
    id: &str,
) -> ZammResult<LlmCall> {
    get_api_call_helper(&database, id).await
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
    struct GetApiCallRequest {
        id: String,
    }

    struct GetApiCallTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<GetApiCallRequest, ZammResult<LlmCall>> for GetApiCallTestCase {
        const EXPECTED_API_CALL: &'static str = "get_api_call";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<GetApiCallRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<LlmCall> {
            get_api_call_helper(
                side_effects.db.as_ref().unwrap(),
                &args.as_ref().unwrap().id,
            )
            .await
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<LlmCall>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&GetApiCallRequest>,
            result: &ZammResult<LlmCall>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<GetApiCallRequest, LlmCall> for GetApiCallTestCase {}

    async fn check_get_api_call_sample(test_fn_name: &'static str, file_prefix: &str) {
        let mut test_case = GetApiCallTestCase { test_fn_name };
        test_case.check_sample_call(file_prefix).await;
    }

    #[tokio::test]
    async fn test_get_api_call_start_conversation() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/get_api_call-start-conversation.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_get_api_call_continued_conversation() {
        check_get_api_call_sample(
            function_name!(),
            "./api/sample-calls/get_api_call-continue-conversation.yaml",
        )
        .await;
    }
}
