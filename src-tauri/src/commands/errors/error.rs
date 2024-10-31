use crate::commands::errors::import::ImportError;
use crate::commands::errors::rodio::RodioError;
use crate::commands::errors::serde::SerdeError;
use crate::setup::api_keys::Service;
use std::sync::PoisonError;

#[derive(thiserror::Error, Debug, specta::Type)]
pub enum Error {
    #[error("Unexpected JSON: {reason}")]
    UnexpectedOpenAiResponse { reason: String },
    #[error("Missing API key for {service}")]
    MissingApiKey { service: Service },
    #[error("Cannot import from ZAMM version {version}. {import_error}")]
    FutureZammImport {
        version: String,
        import_error: ImportError,
    },
    #[error(transparent)]
    GenericImport {
        #[from]
        source: ImportError,
    },
    #[error("Lock poisoned")]
    Poison {},
    #[error(transparent)]
    Serde {
        #[from]
        source: SerdeError,
    },
    #[error(transparent)]
    Rodio {
        #[from]
        source: RodioError,
    },
    #[error("UUID error: {0}")]
    Uuid(String),
    #[error("Diesel error: {0}")]
    Diesel(String),
    #[error("Reqwest error: {0}")]
    Reqwest(String),
    #[error("OpenAI error: {0}")]
    OpenAI(String),
    #[error("Ollama error: {0}")]
    Ollama(String),
    #[error("Tauri error: {0}")]
    Tauri(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("{0}")]
    Other(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poison {}
    }
}

impl From<rodio::StreamError> for Error {
    fn from(err: rodio::StreamError) -> Self {
        let rodio_err: RodioError = err.into();
        rodio_err.into()
    }
}

impl From<rodio::decoder::DecoderError> for Error {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        let rodio_err: RodioError = err.into();
        rodio_err.into()
    }
}

impl From<rodio::PlayError> for Error {
    fn from(err: rodio::PlayError) -> Self {
        let rodio_err: RodioError = err.into();
        rodio_err.into()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        let serde_err: SerdeError = err.into();
        serde_err.into()
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        let serde_err: SerdeError = err.into();
        serde_err.into()
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        let serde_err: SerdeError = err.into();
        serde_err.into()
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        let serde_err: SerdeError = err.into();
        serde_err.into()
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::Uuid(err.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::Diesel(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err.to_string())
    }
}

impl From<async_openai::error::OpenAIError> for Error {
    fn from(err: async_openai::error::OpenAIError) -> Self {
        Self::OpenAI(err.to_string())
    }
}

impl From<ollama_rs::error::OllamaError> for Error {
    fn from(err: ollama_rs::error::OllamaError) -> Self {
        Self::Ollama(err.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(err: tauri::Error) -> Self {
        Self::Tauri(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type ZammResult<T> = std::result::Result<T, Error>;
