use crate::models::llm_calls::chat_message::ChatMessage;
use crate::models::llm_calls::entity_id::EntityId;
use crate::models::llm_calls::llm_call::LlmCall;
use crate::models::llm_calls::row::LlmCallRow;
use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LightweightLlmCall {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub response_message: ChatMessage,
}

impl From<LlmCall> for LightweightLlmCall {
    fn from(value: LlmCall) -> Self {
        LightweightLlmCall {
            id: value.id,
            timestamp: value.timestamp,
            response_message: value.response.completion,
        }
    }
}

impl From<LlmCallRow> for LightweightLlmCall {
    fn from(value: LlmCallRow) -> Self {
        LightweightLlmCall {
            id: value.id,
            timestamp: value.timestamp,
            response_message: value.completion,
        }
    }
}
