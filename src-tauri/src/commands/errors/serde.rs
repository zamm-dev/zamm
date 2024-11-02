#[derive(thiserror::Error, Debug, specta::Type)]
pub enum SerdeError {
    #[error("JSON error: {0}")]
    Json(String),
    #[error("YAML error: {0}")]
    Yaml(String),
    #[error("TOML error: {0}")]
    Toml(String),
}

impl From<serde_json::Error> for SerdeError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
}

impl From<serde_yaml::Error> for SerdeError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Yaml(err.to_string())
    }
}

impl From<toml::de::Error> for SerdeError {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err.to_string())
    }
}

impl From<toml::ser::Error> for SerdeError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Toml(err.to_string())
    }
}
