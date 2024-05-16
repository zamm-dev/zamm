use crate::models::llm_calls::entity_id::EntityId;
use crate::models::llm_calls::row::{LlmCallRow, NewLlmCallRow};
use crate::models::llm_calls::various::{Llm, Request, Response, TokenMetadata};
use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct LlmCall {
    pub id: EntityId,
    pub timestamp: NaiveDateTime,
    pub llm: Llm,
    pub request: Request,
    pub response: Response,
    pub tokens: TokenMetadata,
}

impl LlmCall {
    pub fn as_sql_row(&self) -> NewLlmCallRow {
        NewLlmCallRow {
            id: &self.id,
            timestamp: &self.timestamp,
            provider: &self.llm.provider,
            llm_requested: &self.llm.requested,
            llm: &self.llm.name,
            temperature: &self.request.temperature,
            prompt_tokens: self.tokens.prompt.as_ref(),
            response_tokens: self.tokens.response.as_ref(),
            total_tokens: self.tokens.total.as_ref(),
            prompt: &self.request.prompt,
            completion: &self.response.completion,
        }
    }
}

impl From<LlmCallRow> for LlmCall {
    fn from(row: LlmCallRow) -> Self {
        let id = row.id;
        let timestamp = row.timestamp;
        let llm = Llm {
            name: row.llm,
            requested: row.llm_requested,
            provider: row.provider,
        };
        let request = Request {
            prompt: row.prompt,
            temperature: row.temperature,
        };
        let response = Response {
            completion: row.completion,
        };
        let token_metadata = TokenMetadata {
            prompt: row.prompt_tokens,
            response: row.response_tokens,
            total: row.total_tokens,
        };
        LlmCall {
            id,
            timestamp,
            llm,
            request,
            response,
            tokens: token_metadata,
        }
    }
}
