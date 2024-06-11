use crate::models::llm_calls::entity_id::EntityId;
use crate::schema::{llm_call_follow_ups, llm_call_variants};
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = llm_call_follow_ups)]
pub struct NewLlmCallFollowUp<'a> {
    pub previous_call_id: &'a EntityId,
    pub next_call_id: &'a EntityId,
}

#[derive(Insertable)]
#[diesel(table_name = llm_call_variants)]
pub struct NewLlmCallVariant<'a> {
    pub canonical_id: &'a EntityId,
    pub variant_id: &'a EntityId,
}
