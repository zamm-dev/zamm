use crate::commands::errors::ZammResult;
use crate::commands::Error;
use crate::models::llm_calls::{
    ChatMessage, ChatPrompt, EntityId, LightweightLlmCall, NewLlmCallFollowUp,
    NewLlmCallRow, Prompt, TokenMetadata,
};
use crate::schema::{llm_call_follow_ups, llm_calls};
use crate::setup::api_keys::Service;
use crate::{ZammApiKeys, ZammDatabase};
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs,
};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use specta::specta;
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, specta::Type)]
pub struct ChatArgs {
    provider: Service,
    llm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    prompt: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_call_id: Option<Uuid>,
}

async fn chat_helper(
    zamm_api_keys: &ZammApiKeys,
    zamm_db: &ZammDatabase,
    args: ChatArgs,
    http_client: reqwest_middleware::ClientWithMiddleware,
) -> ZammResult<LightweightLlmCall> {
    let api_keys = zamm_api_keys.0.lock().await;
    if api_keys.openai.is_none() {
        return Err(Error::MissingApiKey {
            service: Service::OpenAI,
        });
    }

    let db = &mut zamm_db.0.lock().await;

    let config = match &args.provider {
        Service::OpenAI => {
            let openai_api_key =
                api_keys.openai.as_ref().ok_or(Error::MissingApiKey {
                    service: Service::OpenAI,
                })?;
            OpenAIConfig::new().with_api_key(openai_api_key)
        }
    };

    let requested_model = args.llm;
    let requested_temperature = args.temperature.unwrap_or(1.0);

    let openai_client =
        async_openai::Client::with_config(config).with_http_client(http_client);
    let messages: Vec<ChatCompletionRequestMessage> =
        args.prompt.clone().into_iter().map(|m| m.into()).collect();
    let request = CreateChatCompletionRequestArgs::default()
        .model(&requested_model)
        .temperature(requested_temperature)
        .messages(messages)
        .build()?;
    let response = openai_client.chat().create(&request).await?;

    let token_metadata = TokenMetadata {
        prompt: response
            .usage
            .as_ref()
            .map(|usage| usage.prompt_tokens as i32),
        response: response
            .usage
            .as_ref()
            .map(|usage| usage.completion_tokens as i32),
        total: response
            .usage
            .as_ref()
            .map(|usage| usage.total_tokens as i32),
    };
    let previous_call_id = args.previous_call_id.map(|id| EntityId { uuid: id });
    let sole_choice = response
        .choices
        .first()
        .ok_or(Error::UnexpectedOpenAiResponse {
            reason: "Zero choices".to_owned(),
        })?
        .message
        .to_owned();

    let new_id = EntityId {
        uuid: Uuid::new_v4(),
    };
    let timestamp = chrono::Utc::now().naive_utc();
    let completion: ChatMessage = sole_choice.try_into()?;

    if let Some(conn) = db.as_mut() {
        diesel::insert_into(llm_calls::table)
            .values(NewLlmCallRow {
                id: &new_id,
                timestamp: &timestamp,
                provider: &args.provider,
                llm_requested: &requested_model,
                llm: &response.model,
                temperature: &requested_temperature,
                prompt_tokens: token_metadata.prompt.as_ref(),
                response_tokens: token_metadata.response.as_ref(),
                total_tokens: token_metadata.total.as_ref(),
                prompt: &Prompt::Chat(ChatPrompt {
                    messages: args.prompt,
                }),
                completion: &completion,
            })
            .execute(conn)?;

        if let Some(previous_id) = previous_call_id {
            diesel::insert_into(llm_call_follow_ups::table)
                .values(NewLlmCallFollowUp {
                    previous_call_id: &previous_id,
                    next_call_id: &new_id,
                })
                .execute(conn)?;
        }
    } // todo: warn users if DB write unsuccessful

    Ok(LightweightLlmCall {
        id: new_id,
        timestamp,
        response_message: completion,
    })
}

#[tauri::command(async)]
#[specta]
pub async fn chat(
    api_keys: State<'_, ZammApiKeys>,
    database: State<'_, ZammDatabase>,
    args: ChatArgs,
) -> ZammResult<LightweightLlmCall> {
    let http_client = reqwest::ClientBuilder::new().build()?;
    let client_with_middleware =
        reqwest_middleware::ClientBuilder::new(http_client).build();
    chat_helper(&api_keys, &database, args, client_with_middleware).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sample_call::SampleCall;
    use crate::setup::api_keys::ApiKeys;
    use crate::test_helpers::api_testing::standard_test_subdir;
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use rvcr::VCRMode;
    use std::collections::HashMap;
    use std::env;
    use stdext::function_name;
    use tokio::sync::Mutex;

    fn to_yaml_string<T: Serialize>(obj: &T) -> String {
        serde_yaml::to_string(obj).unwrap().trim().to_string()
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ChatRequest {
        args: ChatArgs,
    }

    struct ChatTestCase {
        test_fn_name: &'static str,
    }

    impl SampleCallTestCase<ChatRequest, ZammResult<LightweightLlmCall>> for ChatTestCase {
        const EXPECTED_API_CALL: &'static str = "chat";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<ChatRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<LightweightLlmCall> {
            let actual_request = args.as_ref().unwrap().clone();

            let network_helper = side_effects.network.as_ref().unwrap();
            let api_keys = match network_helper.mode {
                VCRMode::Record => ZammApiKeys(Mutex::new(ApiKeys {
                    openai: env::var("OPENAI_API_KEY").ok(),
                })),
                VCRMode::Replay => ZammApiKeys(Mutex::new(ApiKeys {
                    openai: Some("dummy".to_string()),
                })),
            };

            chat_helper(
                &api_keys,
                side_effects.db.as_ref().unwrap(),
                actual_request.args,
                network_helper.network_client.clone(),
            )
            .await
        }

        fn output_replacements(
            &self,
            sample: &SampleCall,
            result: &ZammResult<LightweightLlmCall>,
        ) -> HashMap<String, String> {
            let expected_output = parse_response(&sample.response.message);
            let actual_output = result.as_ref().unwrap();
            let expected_output_timestamp = to_yaml_string(&expected_output.timestamp);
            let actual_output_timestamp = to_yaml_string(&actual_output.timestamp);
            HashMap::from([
                (
                    to_yaml_string(&actual_output.id),
                    to_yaml_string(&expected_output.id),
                ),
                (
                    // sqlite dump produces timestamps with space instead of T
                    actual_output_timestamp.replace('T', " "),
                    expected_output_timestamp.replace('T', " "),
                ),
                (actual_output_timestamp, expected_output_timestamp),
            ])
        }

        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<LightweightLlmCall>,
        ) -> String {
            ZammResultReturn::serialize_result(self, sample, result)
        }

        async fn check_result(
            &self,
            sample: &SampleCall,
            args: Option<&ChatRequest>,
            result: &ZammResult<LightweightLlmCall>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<ChatRequest, LightweightLlmCall> for ChatTestCase {
        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<LightweightLlmCall>,
        ) -> String {
            let expected_llm_call = parse_response(&sample.response.message);
            // swap out non-deterministic parts before JSON comparison
            let deterministic_llm_call = LightweightLlmCall {
                id: expected_llm_call.id,
                timestamp: expected_llm_call.timestamp,
                ..result.as_ref().unwrap().clone()
            };
            serde_json::to_string_pretty(&deterministic_llm_call).unwrap()
        }
    }

    fn parse_response(response_str: &str) -> LightweightLlmCall {
        serde_json::from_str(response_str).unwrap()
    }

    async fn test_llm_api_call(test_fn_name: &'static str, sample_path: &str) {
        let mut test_case = ChatTestCase { test_fn_name };
        test_case.check_sample_call(sample_path).await;
    }

    #[tokio::test]
    async fn test_start_conversation() {
        test_llm_api_call(
            function_name!(),
            "api/sample-calls/chat-start-conversation.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_continue_conversation() {
        test_llm_api_call(
            function_name!(),
            "api/sample-calls/chat-continue-conversation.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_manual_conversation_recreation() {
        test_llm_api_call(
            function_name!(),
            "api/sample-calls/chat-manual-conversation-recreation.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_fork_conversation_step_1() {
        test_llm_api_call(
            function_name!(),
            "api/sample-calls/chat-fork-conversation-python.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_fork_conversation_step_2() {
        test_llm_api_call(
            function_name!(),
            "api/sample-calls/chat-fork-conversation-rust.yaml",
        )
        .await;
    }
}
