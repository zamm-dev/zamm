use crate::models::llm_calls::entity_id::EntityId;
use crate::models::llm_calls::linkage::NewLlmCallContinuation;
use crate::models::llm_calls::row::{LlmCallRow, NewLlmCallRow};
use crate::models::llm_calls::various::{
    ConversationMetadata, Llm, Request, Response, TokenMetadata,
};
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
    #[serde(skip_serializing_if = "ConversationMetadata::is_default", default)]
    pub conversation: ConversationMetadata,
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

    pub fn as_continuation(&self) -> Option<NewLlmCallContinuation> {
        self.conversation
            .previous_call_id
            .as_ref()
            .map(|id| NewLlmCallContinuation {
                previous_call_id: id,
                next_call_id: &self.id,
            })
    }
}

pub type LlmCallLeftJoinResult = (LlmCallRow, Option<EntityId>);
pub type LlmCallQueryResults = (LlmCallLeftJoinResult, Vec<EntityId>);

impl From<LlmCallQueryResults> for LlmCall {
    fn from(query_results: LlmCallQueryResults) -> Self {
        let ((llm_call_row, previous_call_id), next_call_ids) = query_results;

        let id = llm_call_row.id;
        let timestamp = llm_call_row.timestamp;
        let llm = Llm {
            name: llm_call_row.llm,
            requested: llm_call_row.llm_requested,
            provider: llm_call_row.provider,
        };
        let request = Request {
            prompt: llm_call_row.prompt,
            temperature: llm_call_row.temperature,
        };
        let response = Response {
            completion: llm_call_row.completion,
        };
        let token_metadata = TokenMetadata {
            prompt: llm_call_row.prompt_tokens,
            response: llm_call_row.response_tokens,
            total: llm_call_row.total_tokens,
        };
        let conversation_metadata = ConversationMetadata {
            previous_call_id,
            next_call_ids,
        };

        LlmCall {
            id,
            timestamp,
            llm,
            request,
            response,
            tokens: token_metadata,
            conversation: conversation_metadata,
        }
    }
}
