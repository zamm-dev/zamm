use crate::models::llm_calls::chat_message::ChatMessage;
use crate::models::llm_calls::entity_id::EntityId;
use crate::models::llm_calls::prompt::Prompt;
use crate::schema::llm_calls;
use crate::setup::api_keys::Service;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Clone, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = llm_calls)]
pub struct LlmCallRow {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub provider: Service,
    pub llm_requested: String,
    pub llm: String,
    pub temperature: f32,
    pub prompt_tokens: Option<i32>,
    pub response_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
    pub prompt: Prompt,
    pub completion: ChatMessage,
}

impl LlmCallRow {
    pub fn as_insertable(&self) -> NewLlmCallRow {
        NewLlmCallRow {
            id: &self.id,
            timestamp: &self.timestamp,
            provider: &self.provider,
            llm_requested: &self.llm_requested,
            llm: &self.llm,
            temperature: &self.temperature,
            prompt_tokens: self.prompt_tokens.as_ref(),
            response_tokens: self.response_tokens.as_ref(),
            total_tokens: self.total_tokens.as_ref(),
            prompt: &self.prompt,
            completion: &self.completion,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = llm_calls)]
pub struct NewLlmCallRow<'a> {
    pub id: &'a EntityId,
    pub timestamp: &'a NaiveDateTime,
    pub provider: &'a Service,
    pub llm_requested: &'a str,
    pub llm: &'a str,
    pub temperature: &'a f32,
    pub prompt_tokens: Option<&'a i32>,
    pub response_tokens: Option<&'a i32>,
    pub total_tokens: Option<&'a i32>,
    pub prompt: &'a Prompt,
    pub completion: &'a ChatMessage,
}
