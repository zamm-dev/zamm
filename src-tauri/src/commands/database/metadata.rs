#[derive(Debug, Default, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DatabaseCounts {
    pub num_api_keys: i32,
    pub num_llm_calls: i32,
}
