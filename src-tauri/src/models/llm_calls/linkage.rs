use crate::models::llm_calls::entity_id::EntityId;
use crate::schema::{llm_call_follow_ups, llm_call_variants};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[diesel(table_name = llm_call_follow_ups)]
pub struct NewLlmCallFollowUp<'a> {
    pub previous_call_id: &'a EntityId,
    pub next_call_id: &'a EntityId,
}

#[derive(Debug, Queryable, Selectable, Clone, Serialize, Deserialize)]
#[diesel(table_name = llm_call_follow_ups)]
pub struct LlmCallFollowUp {
    pub previous_call_id: EntityId,
    pub next_call_id: EntityId,
}

#[cfg(test)]
impl LlmCallFollowUp {
    pub fn as_insertable(&self) -> NewLlmCallFollowUp {
        NewLlmCallFollowUp {
            previous_call_id: &self.previous_call_id,
            next_call_id: &self.next_call_id,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = llm_call_variants)]
pub struct NewLlmCallVariant<'a> {
    pub canonical_id: &'a EntityId,
    pub variant_id: &'a EntityId,
}

#[derive(Debug, Queryable, Selectable, Clone, Serialize, Deserialize)]
#[diesel(table_name = llm_call_variants)]
pub struct LlmCallVariant {
    pub canonical_id: EntityId,
    pub variant_id: EntityId,
}

#[cfg(test)]
impl LlmCallVariant {
    pub fn as_insertable(&self) -> NewLlmCallVariant {
        NewLlmCallVariant {
            canonical_id: &self.canonical_id,
            variant_id: &self.variant_id,
        }
    }
}
