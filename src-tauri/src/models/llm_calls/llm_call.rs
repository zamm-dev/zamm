use crate::models::llm_calls::chat_message::ChatMessage;
use crate::models::llm_calls::entity_id::EntityId;
use crate::models::llm_calls::row::LlmCallRow;
use crate::models::llm_calls::various::{
    ConversationMetadata, Llm, LlmCallReference, Request, Response, TokenMetadata,
    VariantMetadata,
};
#[cfg(test)]
use crate::models::llm_calls::{NewLlmCallFollowUp, NewLlmCallRow, NewLlmCallVariant};
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
    #[serde(skip_serializing_if = "VariantMetadata::is_default", default)]
    pub variation: VariantMetadata,
}

#[cfg(test)]
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

    pub fn as_follow_up_row(&self) -> Option<NewLlmCallFollowUp> {
        self.conversation
            .previous_call
            .as_ref()
            .map(|call| NewLlmCallFollowUp {
                previous_call_id: &call.id,
                next_call_id: &self.id,
            })
    }

    pub fn as_variant_rows(&self) -> Vec<NewLlmCallVariant> {
        self.variation
            .variants
            .iter()
            .map(|variant| NewLlmCallVariant {
                canonical_id: &self.id,
                variant_id: &variant.id,
            })
            .collect()
    }
}

pub type LlmCallLeftJoinResult = (
    LlmCallRow,
    Option<EntityId>,
    Option<ChatMessage>,
    Option<EntityId>,
    Option<ChatMessage>,
);

pub type LlmCallQueryResults = (
    LlmCallLeftJoinResult,
    Vec<(EntityId, ChatMessage)>,
    Vec<(EntityId, ChatMessage)>,
);

impl From<LlmCallQueryResults> for LlmCall {
    fn from(query_results: LlmCallQueryResults) -> Self {
        let (
            (
                llm_call_row,
                previous_call_id,
                previous_call_completion,
                maybe_canonical_id,
                maybe_canonical_completion,
            ),
            next_calls,
            variants,
        ) = query_results;

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
        let previous_call: Option<LlmCallReference> =
            if let (Some(id), Some(completion)) =
                (previous_call_id, previous_call_completion)
            {
                Some((id, completion).into())
            } else {
                None
            };
        let conversation_metadata = ConversationMetadata {
            previous_call,
            next_calls: next_calls
                .into_iter()
                .map(|(id, completion)| (id, completion).into())
                .collect(),
        };
        let variant_references = variants
            .into_iter()
            .map(|(id, completion)| (id, completion).into())
            .collect();
        let variant_metadata = if let (Some(canonical_id), Some(canonical_completion)) =
            (maybe_canonical_id, maybe_canonical_completion)
        {
            VariantMetadata {
                canonical: Some((canonical_id, canonical_completion).into()),
                variants: Vec::new(),
                sibling_variants: variant_references,
            }
        } else {
            VariantMetadata {
                canonical: None,
                variants: variant_references,
                sibling_variants: Vec::new(),
            }
        };

        LlmCall {
            id,
            timestamp,
            llm,
            request,
            response,
            tokens: token_metadata,
            conversation: conversation_metadata,
            variation: variant_metadata,
        }
    }
}
