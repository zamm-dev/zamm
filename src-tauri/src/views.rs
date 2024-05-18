use crate::schema::llm_calls;

diesel::table! {
    llm_call_named_continuations (previous_call_id, next_call_id) {
        previous_call_id -> Text,
        previous_call_completion -> Text,
        next_call_id -> Text,
        next_call_completion -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(llm_call_named_continuations, llm_calls);
