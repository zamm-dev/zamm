use crate::models::llm_calls::entity_id::EntityId;
use crate::schema::llm_call_continuations;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = llm_call_continuations)]
pub struct NewLlmCallContinuation<'a> {
    pub previous_call_id: &'a EntityId,
    pub next_call_id: &'a EntityId,
}
