use crate::commands::errors::ZammResult;
use crate::commands::Error;
use crate::models::llm_calls::{
    ChatMessage, ChatPrompt, EntityId, Llm, LlmCall, Prompt, Request, Response,
    TokenMetadata,
};
use crate::schema::llm_calls;
use crate::setup::api_keys::Service;
use crate::{ZammApiKeys, ZammDatabase};
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs,
};
use diesel::RunQueryDsl;
use specta::specta;
use tauri::State;
use uuid::Uuid;

async fn chat_helper(
    zamm_api_keys: &ZammApiKeys,
    zamm_db: &ZammDatabase,
    provider: Service,
    llm: String,
    temperature: Option<f32>,
    prompt: Vec<ChatMessage>,
    http_client: reqwest_middleware::ClientWithMiddleware,
) -> ZammResult<LlmCall> {
    let api_keys = zamm_api_keys.0.lock().await;
    if api_keys.openai.is_none() {
        return Err(Error::MissingApiKey {
            service: Service::OpenAI,
        });
    }

    let db = &mut zamm_db.0.lock().await;

    let config = match provider {
        Service::OpenAI => {
            let openai_api_key =
                api_keys.openai.as_ref().ok_or(Error::MissingApiKey {
                    service: Service::OpenAI,
                })?;
            OpenAIConfig::new().with_api_key(openai_api_key)
        }
    };

    let requested_model = llm;
    let requested_temperature = temperature.unwrap_or(1.0);

    let openai_client =
        async_openai::Client::with_config(config).with_http_client(http_client);
    let messages: Vec<ChatCompletionRequestMessage> =
        prompt.clone().into_iter().map(|m| m.into()).collect();
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
    let sole_choice = response
        .choices
        .first()
        .ok_or(Error::UnexpectedOpenAiResponse {
            reason: "Zero choices".to_owned(),
        })?
        .message
        .to_owned();
    let llm_call = LlmCall {
        id: EntityId {
            uuid: Uuid::new_v4(),
        },
        timestamp: chrono::Utc::now().naive_utc(),
        llm: Llm {
            provider: Service::OpenAI,
            name: response.model.clone(),
            requested: requested_model.to_owned(),
        },
        request: Request {
            temperature: requested_temperature,
            prompt: Prompt::Chat(ChatPrompt { messages: prompt }),
        },
        response: Response {
            completion: sole_choice.try_into()?,
        },
        tokens: token_metadata,
    };

    if let Some(conn) = db.as_mut() {
        diesel::insert_into(llm_calls::table)
            .values(llm_call.as_sql_row())
            .execute(conn)?;
    } // todo: warn users if DB write unsuccessful

    Ok(llm_call)
}

#[tauri::command(async)]
#[specta]
pub async fn chat(
    api_keys: State<'_, ZammApiKeys>,
    database: State<'_, ZammDatabase>,
    provider: Service,
    llm: String,
    temperature: Option<f32>,
    prompt: Vec<ChatMessage>,
) -> ZammResult<LlmCall> {
    let http_client = reqwest::ClientBuilder::new().build()?;
    let client_with_middleware =
        reqwest_middleware::ClientBuilder::new(http_client).build();
    chat_helper(
        &api_keys,
        &database,
        provider,
        llm,
        temperature,
        prompt,
        client_with_middleware,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::llm_calls::ChatMessage;
    use crate::sample_call::SampleCall;
    use crate::setup::api_keys::ApiKeys;
    use crate::test_helpers::api_testing::standard_test_subdir;
    use crate::test_helpers::{
        SampleCallTestCase, SideEffectsHelpers, ZammResultReturn,
    };
    use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
    use rvcr::{VCRMiddleware, VCRMode};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::env;
    use std::path::PathBuf;
    use stdext::function_name;
    use tokio::sync::Mutex;
    use vcr_cassette::Headers;

    const CENSORED: &str = "<CENSORED>";

    fn censor_headers(headers: &Headers, blacklisted_keys: &[&str]) -> Headers {
        return headers
            .clone()
            .iter()
            .map(|(k, v)| {
                if blacklisted_keys.contains(&k.as_str()) {
                    (k.clone(), vec![CENSORED.to_string()])
                } else {
                    (k.clone(), v.clone())
                }
            })
            .collect();
    }

    fn to_yaml_string<T: Serialize>(obj: &T) -> String {
        serde_yaml::to_string(obj).unwrap().trim().to_string()
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ChatRequest {
        provider: Service,
        llm: String,
        temperature: Option<f32>,
        prompt: Vec<ChatMessage>,
    }

    struct ChatTestCase {
        test_fn_name: &'static str,
        pub api_keys: ZammApiKeys,
        pub vcr_client: ClientWithMiddleware,
    }

    impl SampleCallTestCase<ChatRequest, ZammResult<LlmCall>> for ChatTestCase {
        const EXPECTED_API_CALL: &'static str = "chat";
        const CALL_HAS_ARGS: bool = true;

        fn temp_test_subdirectory(&self) -> String {
            standard_test_subdir(Self::EXPECTED_API_CALL, self.test_fn_name)
        }

        async fn make_request(
            &mut self,
            args: &Option<ChatRequest>,
            side_effects: &SideEffectsHelpers,
        ) -> ZammResult<LlmCall> {
            let actual_args = args.as_ref().unwrap().clone();
            chat_helper(
                &self.api_keys,
                side_effects.db.as_ref().unwrap(),
                actual_args.provider,
                actual_args.llm,
                actual_args.temperature,
                actual_args.prompt,
                self.vcr_client.clone(),
            )
            .await
        }

        fn output_replacements(
            &self,
            sample: &SampleCall,
            result: &ZammResult<LlmCall>,
        ) -> HashMap<String, String> {
            let expected_output = parse_response(&sample.response.message);
            let actual_output = result.as_ref().unwrap();
            HashMap::from([
                (
                    to_yaml_string(&actual_output.id),
                    to_yaml_string(&expected_output.id),
                ),
                (
                    to_yaml_string(&actual_output.timestamp),
                    to_yaml_string(&expected_output.timestamp),
                ),
            ])
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
            args: Option<&ChatRequest>,
            result: &ZammResult<LlmCall>,
        ) {
            ZammResultReturn::check_result(self, sample, args, result).await
        }
    }

    impl ZammResultReturn<ChatRequest, LlmCall> for ChatTestCase {
        fn serialize_result(
            &self,
            sample: &SampleCall,
            result: &ZammResult<LlmCall>,
        ) -> String {
            let expected_llm_call = parse_response(&sample.response.message);
            // swap out non-deterministic parts before JSON comparison
            let deterministic_llm_call = LlmCall {
                id: expected_llm_call.id,
                timestamp: expected_llm_call.timestamp,
                ..result.as_ref().unwrap().clone()
            };
            serde_json::to_string_pretty(&deterministic_llm_call).unwrap()
        }
    }

    fn parse_response(response_str: &str) -> LlmCall {
        serde_json::from_str(response_str).unwrap()
    }

    async fn test_llm_api_call(
        test_fn_name: &'static str,
        recording_path: &str,
        sample_path: &str,
    ) {
        let recording_path = PathBuf::from(recording_path);
        let is_recording = !recording_path.exists();
        let api_keys = if is_recording {
            ZammApiKeys(Mutex::new(ApiKeys {
                openai: env::var("OPENAI_API_KEY").ok(),
            }))
        } else {
            ZammApiKeys(Mutex::new(ApiKeys {
                openai: Some("dummy".to_string()),
            }))
        };

        let vcr_mode = if is_recording {
            VCRMode::Record
        } else {
            VCRMode::Replay
        };
        let middleware = VCRMiddleware::try_from(recording_path)
            .unwrap()
            .with_mode(vcr_mode)
            .with_modify_request(|req| {
                req.headers = censor_headers(&req.headers, &["authorization"]);
            })
            .with_modify_response(|resp| {
                resp.headers = censor_headers(&resp.headers, &["openai-organization"]);
            });

        let vcr_client: ClientWithMiddleware =
            ClientBuilder::new(reqwest::Client::new())
                .with(middleware)
                .build();
        // end dependencies setup

        let mut test_case = ChatTestCase {
            test_fn_name,
            api_keys,
            vcr_client,
        };
        test_case.check_sample_call(sample_path).await;
    }

    #[tokio::test]
    async fn test_start_conversation() {
        test_llm_api_call(
            function_name!(),
            "api/sample-call-requests/start-conversation.json",
            "api/sample-calls/chat-start-conversation.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_continue_conversation() {
        test_llm_api_call(
            function_name!(),
            "api/sample-call-requests/continue-conversation.json",
            "api/sample-calls/chat-continue-conversation.yaml",
        )
        .await;
    }
}
