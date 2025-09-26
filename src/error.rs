use hyperware_process_lib::http::client::HttpClientError;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum TtsError {
    #[error("missing input text")]
    MissingInput,

    #[error("input text too long: {0} characters (max: 5000)")]
    InputTooLong(usize),

    #[error("invalid voice setting {field}: {value} (must be between 0.0 and 1.0)")]
    InvalidVoiceSettings { field: String, value: f32 },

    #[error("missing API key")]
    MissingApiKey,

    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] HttpClientError),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("invalid seed value: {0} (must be between 0 and 4294967295)")]
    InvalidSeed(u32),
}
