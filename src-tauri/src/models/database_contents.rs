use crate::models::asciicasts::{AsciiCast, NewAsciiCast};
use crate::models::llm_calls::{
    LlmCallFollowUp, LlmCallRow, LlmCallVariant, NewLlmCallFollowUp, NewLlmCallRow,
    NewLlmCallVariant,
};
use crate::models::{ApiKey, NewApiKey};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LlmCallData {
    pub instances: Vec<LlmCallRow>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub follow_ups: Vec<LlmCallFollowUp>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub variants: Vec<LlmCallVariant>,
}

impl LlmCallData {
    pub fn is_default(&self) -> bool {
        self.instances.is_empty()
            && self.follow_ups.is_empty()
            && self.variants.is_empty()
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DatabaseContents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zamm_version: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub api_keys: Vec<ApiKey>,
    #[serde(skip_serializing_if = "LlmCallData::is_default", default)]
    pub llm_calls: LlmCallData,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub terminal_sessions: Vec<AsciiCast>,
}

impl DatabaseContents {
    pub fn insertable_api_keys(&self) -> Vec<NewApiKey> {
        self.api_keys.iter().map(|k| k.as_insertable()).collect()
    }

    pub fn insertable_llm_calls(&self) -> Vec<NewLlmCallRow> {
        self.llm_calls
            .instances
            .iter()
            .map(|k| k.as_insertable())
            .collect()
    }

    pub fn insertable_call_follow_ups(&self) -> Vec<NewLlmCallFollowUp> {
        self.llm_calls
            .follow_ups
            .iter()
            .map(|k| k.as_insertable())
            .collect()
    }

    pub fn insertable_call_variants(&self) -> Vec<NewLlmCallVariant> {
        self.llm_calls
            .variants
            .iter()
            .map(|k| k.as_insertable())
            .collect()
    }

    pub fn insertable_terminal_sessions(&self) -> Vec<NewAsciiCast> {
        self.terminal_sessions
            .iter()
            .map(|k| k.as_insertable())
            .collect()
    }
}
