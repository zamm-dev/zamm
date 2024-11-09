fn is_zero(num: &i32) -> bool {
    *num == 0
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DatabaseCounts {
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub num_api_keys: i32,
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub num_llm_calls: i32,
    #[serde(skip_serializing_if = "is_zero")]
    #[serde(default)]
    pub num_terminal_sessions: i32,
}

impl DatabaseCounts {
    pub fn is_empty(&self) -> bool {
        self.num_api_keys == 0
            && self.num_llm_calls == 0
            && self.num_terminal_sessions == 0
    }
}
