pub mod client;
pub mod error;
pub mod types;

pub use client::{SpeechClient, SpeechRequestBuilder};
pub use error::TtsError;
pub use types::{
    AudioFormat, SpeechRequest, SpeechResponse, TextNormalization, TtsModel, Voice, VoiceSettings,
};
