use crate::schema::llm_calls;

diesel::table! {
    llm_call_named_follow_ups (previous_call_id, next_call_id) {
        previous_call_id -> Text,
        previous_call_completion -> Text,
        next_call_id -> Text,
        next_call_completion -> Text,
    }
}

diesel::table! {
    llm_call_named_variants (canonical_id, variant_id) {
        canonical_id -> Text,
        canonical_completion -> Text,
        variant_id -> Text,
        variant_completion -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    llm_call_named_follow_ups,
    llm_call_named_variants,
    llm_calls
);
