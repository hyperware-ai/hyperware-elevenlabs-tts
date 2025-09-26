use crate::error::TtsError;
use crate::types::{
    ApiErrorResponse, AudioFormat, SpeechRequest, SpeechRequestJson, SpeechResponse,
    TextNormalization, TtsModel, Voice, VoiceSettings,
};
use hyperware_process_lib::http::client::{send_request_await_response, HttpClientError};
use http::Method;
use std::collections::HashMap;

const MAX_INPUT_LENGTH: usize = 5000;
const MIN_VOICE_SETTING: f32 = 0.0;
const MAX_VOICE_SETTING: f32 = 1.0;

pub struct SpeechClient {
    api_key: String,
    base_url: String,
    timeout: u64,
}

impl SpeechClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io".to_string(),
            timeout: 60000,
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn synthesize(&self) -> SpeechRequestBuilder {
        SpeechRequestBuilder {
            client: self,
            request: SpeechRequest::default(),
        }
    }

    async fn send_speech_request(
        &self,
        request: SpeechRequest,
    ) -> Result<SpeechResponse, TtsError> {
        if request.text.is_empty() {
            return Err(TtsError::MissingInput);
        }

        if request.text.len() > MAX_INPUT_LENGTH {
            return Err(TtsError::InputTooLong(request.text.len()));
        }

        if let Some(ref settings) = request.voice_settings {
            if let Some(stability) = settings.stability {
                if stability < MIN_VOICE_SETTING || stability > MAX_VOICE_SETTING {
                    return Err(TtsError::InvalidVoiceSettings {
                        field: "stability".to_string(),
                        value: stability,
                    });
                }
            }
            if let Some(similarity_boost) = settings.similarity_boost {
                if similarity_boost < MIN_VOICE_SETTING || similarity_boost > MAX_VOICE_SETTING {
                    return Err(TtsError::InvalidVoiceSettings {
                        field: "similarity_boost".to_string(),
                        value: similarity_boost,
                    });
                }
            }
            if let Some(style) = settings.style {
                if style < MIN_VOICE_SETTING || style > MAX_VOICE_SETTING {
                    return Err(TtsError::InvalidVoiceSettings {
                        field: "style".to_string(),
                        value: style,
                    });
                }
            }
        }

        if self.api_key.is_empty() {
            return Err(TtsError::MissingApiKey);
        }

        let json_request = SpeechRequestJson::from(request.clone());

        let body = serde_json::to_vec(&json_request)
            .map_err(|e| TtsError::SerializationError(e.to_string()))?;

        let mut headers = HashMap::new();
        headers.insert("xi-api-key".to_string(), self.api_key.clone());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let voice_id = request.voice.as_voice_id();
        let default_format = AudioFormat::default();
        let output_format = request
            .output_format
            .as_ref()
            .unwrap_or(&default_format)
            .as_str();

        let url = url::Url::parse(&format!(
            "{}/v1/text-to-speech/{}?output_format={}",
            self.base_url,
            voice_id,
            output_format
        ))
        .map_err(|e| {
            TtsError::HttpClient(HttpClientError::BadUrl {
                url: e.to_string(),
            })
        })?;

        let response = send_request_await_response(
            Method::POST,
            url,
            Some(headers),
            self.timeout,
            body,
        )
        .await
        .map_err(TtsError::HttpClient)?;

        let status = response.status();
        let body = response.into_body();

        if status.is_success() {
            let format = request.output_format.unwrap_or_default();
            Ok(SpeechResponse {
                audio_data: body,
                format,
            })
        } else {
            if let Ok(error_response) = serde_json::from_slice::<ApiErrorResponse>(&body) {
                Err(TtsError::ApiError {
                    status: status.as_u16(),
                    message: error_response.error.message,
                })
            } else {
                let message = String::from_utf8_lossy(&body).to_string();
                Err(TtsError::ApiError {
                    status: status.as_u16(),
                    message,
                })
            }
        }
    }
}

pub struct SpeechRequestBuilder<'a> {
    client: &'a SpeechClient,
    request: SpeechRequest,
}

impl<'a> SpeechRequestBuilder<'a> {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.request.text = text.into();
        self
    }

    pub fn input(self, text: impl Into<String>) -> Self {
        self.text(text)
    }

    pub fn model(mut self, model: TtsModel) -> Self {
        self.request.model = model;
        self
    }

    pub fn voice(mut self, voice: Voice) -> Self {
        self.request.voice = voice;
        self
    }

    pub fn voice_settings(mut self, settings: VoiceSettings) -> Self {
        self.request.voice_settings = Some(settings);
        self
    }

    pub fn stability(mut self, stability: f32) -> Self {
        let mut settings = self.request.voice_settings.unwrap_or_default();
        settings.stability = Some(stability);
        self.request.voice_settings = Some(settings);
        self
    }

    pub fn similarity_boost(mut self, similarity_boost: f32) -> Self {
        let mut settings = self.request.voice_settings.unwrap_or_default();
        settings.similarity_boost = Some(similarity_boost);
        self.request.voice_settings = Some(settings);
        self
    }

    pub fn style(mut self, style: f32) -> Self {
        let mut settings = self.request.voice_settings.unwrap_or_default();
        settings.style = Some(style);
        self.request.voice_settings = Some(settings);
        self
    }

    pub fn use_speaker_boost(mut self, use_speaker_boost: bool) -> Self {
        let mut settings = self.request.voice_settings.unwrap_or_default();
        settings.use_speaker_boost = Some(use_speaker_boost);
        self.request.voice_settings = Some(settings);
        self
    }

    pub fn output_format(mut self, format: AudioFormat) -> Self {
        self.request.output_format = Some(format);
        self
    }

    pub fn response_format(self, format: AudioFormat) -> Self {
        self.output_format(format)
    }

    pub fn language_code(mut self, code: impl Into<String>) -> Self {
        self.request.language_code = Some(code.into());
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.request.seed = Some(seed);
        self
    }

    pub fn previous_text(mut self, text: impl Into<String>) -> Self {
        self.request.previous_text = Some(text.into());
        self
    }

    pub fn next_text(mut self, text: impl Into<String>) -> Self {
        self.request.next_text = Some(text.into());
        self
    }

    pub fn previous_request_ids(mut self, ids: Vec<String>) -> Self {
        self.request.previous_request_ids = Some(ids);
        self
    }

    pub fn next_request_ids(mut self, ids: Vec<String>) -> Self {
        self.request.next_request_ids = Some(ids);
        self
    }

    pub fn apply_text_normalization(mut self, mode: TextNormalization) -> Self {
        self.request.apply_text_normalization = Some(mode);
        self
    }

    pub fn apply_language_text_normalization(mut self, enabled: bool) -> Self {
        self.request.apply_language_text_normalization = Some(enabled);
        self
    }

    pub async fn execute(self) -> Result<SpeechResponse, TtsError> {
        self.client.send_speech_request(self.request).await
    }
}
