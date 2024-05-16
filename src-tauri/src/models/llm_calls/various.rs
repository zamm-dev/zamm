use crate::models::llm_calls::chat_message::ChatMessage;
use crate::models::llm_calls::prompt::Prompt;
use crate::setup::api_keys::Service;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, specta::Type)]
pub struct Llm {
    pub name: String,
    pub requested: String,
    pub provider: Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Request {
    pub prompt: Prompt,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Response {
    pub completion: ChatMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TokenMetadata {
    pub prompt: Option<i32>,
    pub response: Option<i32>,
    pub total: Option<i32>,
}
