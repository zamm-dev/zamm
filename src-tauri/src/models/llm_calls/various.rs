use crate::models::llm_calls::chat_message::ChatMessage;
use crate::models::llm_calls::entity_id::EntityId;
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

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ConversationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_call_id: Option<EntityId>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub next_call_ids: Vec<EntityId>,
}
