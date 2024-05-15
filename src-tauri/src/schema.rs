// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (service) {
        service -> Text,
        api_key -> Text,
    }
}

diesel::table! {
    llm_call_continuations (previous_call_id, next_call_id) {
        previous_call_id -> Text,
        next_call_id -> Text,
    }
}

diesel::table! {
    llm_calls (id) {
        id -> Text,
        timestamp -> Timestamp,
        provider -> Text,
        llm_requested -> Text,
        llm -> Text,
        temperature -> Float,
        prompt_tokens -> Nullable<Integer>,
        response_tokens -> Nullable<Integer>,
        total_tokens -> Nullable<Integer>,
        prompt -> Text,
        completion -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    llm_call_continuations,
    llm_calls,
);
