use crate::setup::api_keys::Service;
use std::{fmt, sync::PoisonError};

#[derive(Debug)]
pub struct SidecarResponseError {
    pub request: Vec<String>,
    pub response: String,
    pub source: serde_json::Error,
}

impl std::error::Error for SidecarResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

impl fmt::Display for SidecarResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut json_err_msg = String::new();

        json_err_msg.push_str("Failed to parse sidecar JSON.\n");

        json_err_msg.push_str("Request: ");
        for arg in self.request.iter() {
            json_err_msg.push_str(arg);
            json_err_msg.push(' ');
        }
        json_err_msg.push('\n');

        json_err_msg.push_str("Response: ");
        json_err_msg.push_str(&self.response);
        json_err_msg.push('\n');

        json_err_msg.push_str("Error: ");
        json_err_msg.push_str(&self.source.to_string());
        write!(f, "{}", json_err_msg)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RodioError {
    #[error(transparent)]
    Stream {
        #[from]
        source: rodio::StreamError,
    },
    #[error(transparent)]
    Decode {
        #[from]
        source: rodio::decoder::DecoderError,
    },
    #[error(transparent)]
    Play {
        #[from]
        source: rodio::PlayError,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum SerdeError {
    #[error(transparent)]
    Json {
        #[from]
        source: serde_json::Error,
    },
    #[error(transparent)]
    TomlDeserialize {
        #[from]
        source: toml::de::Error,
    },
    #[error(transparent)]
    TomlSerialize {
        #[from]
        source: toml::ser::Error,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to spawn sidecar at {expected_path}: {tauri_error}")]
    SidecarSpawn {
        expected_path: String,
        tauri_error: tauri::api::Error,
    },
    #[error(transparent)]
    SidecarResponse {
        #[from]
        source: SidecarResponseError,
    },
    #[error("Sidecar command error event: {line}")]
    SidecarCommandErr { line: String },
    #[error("Unexpected sidecar command event")]
    SidecarUnexpectedCommandEvent,
    #[error("Unexpected JSON: {reason}")]
    UnexpectedOpenAiResponse { reason: String },
    #[error("Missing API key for {service}")]
    MissingApiKey { service: Service },
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
    #[error(transparent)]
    Diesel {
        #[from]
        source: diesel::result::Error,
    },
    #[error(transparent)]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error(transparent)]
    OpenAI {
        #[from]
        source: async_openai::error::OpenAIError,
    },
    #[error(transparent)]
    Tauri {
        #[from]
        source: tauri::Error,
    },
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Unknown failure: {source}")]
    Other {
        #[from]
        source: anyhow::Error,
    },
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

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type ZammResult<T> = std::result::Result<T, Error>;
