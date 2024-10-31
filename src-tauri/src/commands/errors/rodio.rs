#[derive(thiserror::Error, Debug, specta::Type)]
pub enum RodioError {
    #[error("Stream error: {0}")]
    Stream(String),
    #[error("Decoder error: {0}")]
    Decode(String),
    #[error("Play error: {0}")]
    Play(String),
}

impl From<rodio::StreamError> for RodioError {
    fn from(err: rodio::StreamError) -> Self {
        Self::Stream(err.to_string())
    }
}

impl From<rodio::decoder::DecoderError> for RodioError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self::Decode(err.to_string())
    }
}

impl From<rodio::PlayError> for RodioError {
    fn from(err: rodio::PlayError) -> Self {
        Self::Play(err.to_string())
    }
}
