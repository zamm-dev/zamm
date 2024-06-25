use crate::commands::errors::ZammResult;
use crate::models::llm_calls::{ChatMessage, EntityId, LlmCall, LlmCallLeftJoinResult};
use crate::schema::llm_calls;
use crate::views::{llm_call_named_follow_ups, llm_call_named_variants};
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

    let left_join_result: LlmCallLeftJoinResult = llm_calls::table
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
        .filter(llm_calls::id.eq(&parsed_uuid))
        .first::<LlmCallLeftJoinResult>(conn)?;
    let next_calls_result = llm_call_named_follow_ups::table
        .select((
            llm_call_named_follow_ups::next_call_id,
            llm_call_named_follow_ups::next_call_completion,
        ))
        .filter(llm_call_named_follow_ups::previous_call_id.eq(&parsed_uuid))
        .load::<(EntityId, ChatMessage)>(conn)?;
    let canonical_id = left_join_result.3.clone().unwrap_or(parsed_uuid);
    let variants_result = llm_call_named_variants::table
        .select((
            llm_call_named_variants::variant_id,
            llm_call_named_variants::variant_completion,
        ))
        .filter(llm_call_named_variants::canonical_id.eq(canonical_id))
        .load::<(EntityId, ChatMessage)>(conn)?;
    Ok((left_join_result, next_calls_result, variants_result).into())
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
    use crate::test_helpers::SideEffectsHelpers;
    use crate::{check_sample, impl_result_test_case};

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct GetApiCallRequest {
        id: String,
    }

    async fn make_request_helper(
        args: &GetApiCallRequest,
        side_effects: &SideEffectsHelpers,
    ) -> ZammResult<LlmCall> {
        get_api_call_helper(side_effects.db.as_ref().unwrap(), &args.id).await
    }

    impl_result_test_case!(
        GetApiCallTestCase,
        get_api_call,
        true,
        GetApiCallRequest,
        LlmCall
    );

    check_sample!(
        GetApiCallTestCase,
        test_no_links,
        "./api/sample-calls/get_api_call-no-links.yaml"
    );

    check_sample!(
        GetApiCallTestCase,
        test_start_conversation,
        "./api/sample-calls/get_api_call-start-conversation.yaml"
    );

    check_sample!(
        GetApiCallTestCase,
        test_continued_conversation,
        "./api/sample-calls/get_api_call-continue-conversation.yaml"
    );

    check_sample!(
        GetApiCallTestCase,
        test_edit,
        "./api/sample-calls/get_api_call-edit.yaml"
    );
}
