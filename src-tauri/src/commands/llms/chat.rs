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
    use crate::models::llm_calls::{ChatMessage, LlmCallRow};
    use crate::sample_call::SampleCall;
    use crate::setup::api_keys::ApiKeys;
    use crate::setup::db::MIGRATIONS;
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
    use rvcr::{VCRMiddleware, VCRMode};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
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

    fn setup_database() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    pub fn setup_zamm_db() -> ZammDatabase {
        ZammDatabase(Mutex::new(Some(setup_database())))
    }

    async fn get_llm_call(db: &ZammDatabase, call_id: &EntityId) -> LlmCall {
        use crate::schema::llm_calls::dsl::*;
        let mut conn_mutex = db.0.lock().await;
        let conn = conn_mutex.as_mut().unwrap();
        llm_calls
            .filter(id.eq(call_id))
            .first::<LlmCallRow>(conn)
            .unwrap()
            .into()
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ChatRequest {
        provider: Service,
        llm: String,
        temperature: Option<f32>,
        prompt: Vec<ChatMessage>,
    }

    fn parse_request(request_str: &str) -> ChatRequest {
        serde_json::from_str(request_str).unwrap()
    }

    fn parse_response(response_str: &str) -> LlmCall {
        serde_json::from_str(response_str).unwrap()
    }

    fn read_sample(filename: &str) -> SampleCall {
        let sample_str = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("No file found at {filename}"));
        serde_yaml::from_str(&sample_str).unwrap()
    }

    async fn test_llm_api_call(recording_path: &str, sample_path: &str) {
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

        let db = setup_zamm_db();
        // end dependencies setup

        let sample = read_sample(sample_path);
        assert_eq!(sample.request.len(), 2);
        assert_eq!(sample.request[0], "chat");

        let request = parse_request(&sample.request[1]);

        let result = chat_helper(
            &api_keys,
            &db,
            request.provider,
            request.llm,
            request.temperature,
            request.prompt,
            vcr_client,
        )
        .await;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        let ok_result = result.unwrap();

        // check that the API call returns the expected JSON
        let expected_llm_call = parse_response(&sample.response.message);
        // swap out non-deterministic parts before JSON comparison
        let deterministic_llm_call = LlmCall {
            id: expected_llm_call.id,
            timestamp: expected_llm_call.timestamp,
            ..ok_result.clone()
        };
        let actual_json =
            serde_json::to_string_pretty(&deterministic_llm_call).unwrap();
        let expected_json = sample.response.message.trim();
        assert_eq!(actual_json, expected_json);

        // check that it made it into the database
        let stored_llm_call = get_llm_call(&db, &ok_result.id).await;
        assert_eq!(stored_llm_call.request.prompt, ok_result.request.prompt);
        assert_eq!(
            stored_llm_call.response.completion,
            ok_result.response.completion
        );

        // do a sanity check that everything is non-empty
        let prompt = match ok_result.request.prompt {
            Prompt::Chat(ChatPrompt { messages: prompt }) => prompt,
        };
        assert!(!prompt.is_empty());
        match &ok_result.response.completion {
            ChatMessage::AI { text } => assert!(!text.is_empty()),
            _ => panic!("Unexpected response type"),
        }
    }

    #[tokio::test]
    async fn test_start_conversation() {
        test_llm_api_call(
            "api/sample-call-requests/start-conversation.json",
            "api/sample-calls/chat-start-conversation.yaml",
        )
        .await;
    }

    #[tokio::test]
    async fn test_continue_conversation() {
        test_llm_api_call(
            "api/sample-call-requests/continue-conversation.json",
            "api/sample-calls/chat-continue-conversation.yaml",
        )
        .await;
    }
}
