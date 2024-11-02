#[derive(thiserror::Error, Debug, specta::Type)]
pub enum ImportError {
    #[error("Data contains unknown prompt types.")]
    UnknownPromptType {},
}
