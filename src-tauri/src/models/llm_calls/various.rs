use crate::models::llm_calls::chat_message::ChatMessage;
use crate::models::llm_calls::entity_id::EntityId;
use crate::models::llm_calls::prompt::Prompt;
use crate::setup::api_keys::Service;
use serde::{Deserialize, Serialize};

const NUM_WORDS_TO_SNIPPET: usize = 20;

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
pub struct LlmCallReference {
    pub id: EntityId,
    pub snippet: String,
}

impl From<(EntityId, ChatMessage)> for LlmCallReference {
    fn from((id, message): (EntityId, ChatMessage)) -> Self {
        let text = match message {
            ChatMessage::System { text } => text,
            ChatMessage::Human { text } => text,
            ChatMessage::AI { text } => text,
        };
        let truncated_text = text
            .split_whitespace()
            .take(NUM_WORDS_TO_SNIPPET)
            .collect::<Vec<&str>>()
            .join(" ");
        let snippet = if text == truncated_text {
            text
        } else {
            format!("{}...", truncated_text)
        };
        Self { id, snippet }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, specta::Type)]
pub struct ConversationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_call: Option<LlmCallReference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub next_calls: Vec<LlmCallReference>,
}

impl ConversationMetadata {
    pub fn is_default(&self) -> bool {
        self.previous_call.is_none() && self.next_calls.is_empty()
    }
}
