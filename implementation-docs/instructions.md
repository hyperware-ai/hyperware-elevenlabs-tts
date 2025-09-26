# 250926

## ElevenLabs TTS crate

*Hyperware OpenAI TTS api*

client.rs
```rust
use crate::error::TtsError;
use crate::types::{
    ApiErrorResponse, AudioFormat, SpeechRequest, SpeechRequestJson, SpeechResponse, TtsModel,
    Voice,
};
use hyperware_process_lib::http::client::send_request_await_response;
use hyperware_process_lib::http::client::HttpClientError;
use http::Method;
use std::collections::HashMap;

const MAX_INPUT_LENGTH: usize = 4096;
const MIN_SPEED: f32 = 0.25;
const MAX_SPEED: f32 = 4.0;

pub struct SpeechClient {
    api_key: String,
    base_url: String,
    timeout: u64,
}

impl SpeechClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com".to_string(),
            timeout: 60000, // 60 seconds default
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
        // Validate input
        if request.input.is_empty() {
            return Err(TtsError::MissingInput);
        }

        if request.input.len() > MAX_INPUT_LENGTH {
            return Err(TtsError::InputTooLong(request.input.len()));
        }

        if let Some(speed) = request.speed {
            if speed < MIN_SPEED || speed > MAX_SPEED {
                return Err(TtsError::InvalidSpeed(speed));
            }
        }

        if self.api_key.is_empty() {
            return Err(TtsError::MissingApiKey);
        }

        // Convert to JSON request
        let json_request = SpeechRequestJson::from(request.clone());
        
        // Serialize to JSON
        let body = serde_json::to_vec(&json_request)
            .map_err(|e| TtsError::SerializationError(e.to_string()))?;

        // Prepare headers
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        // Construct URL
        let url = url::Url::parse(&format!("{}/v1/audio/speech", self.base_url))
            .map_err(|e| TtsError::HttpClient(HttpClientError::BadUrl {
                url: e.to_string(),
            }))?;

        // Send request
        let response = send_request_await_response(
            Method::POST,
            url,
            Some(headers),
            self.timeout,
            body,
        )
        .await
        .map_err(TtsError::HttpClient)?;

        // Handle response
        let status = response.status();
        let body = response.into_body();

        if status.is_success() {
            // Success - body contains raw audio data
            let format = request.response_format.unwrap_or(AudioFormat::Mp3);
            Ok(SpeechResponse {
                audio_data: body,
                format,
            })
        } else {
            // Try to parse error response
            if let Ok(error_response) = serde_json::from_slice::<ApiErrorResponse>(&body) {
                Err(TtsError::ApiError {
                    status: status.as_u16(),
                    message: error_response.error.message,
                })
            } else {
                // Fallback to raw body text
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
    pub fn input(mut self, text: impl Into<String>) -> Self {
        self.request.input = text.into();
        self
    }

    pub fn model(mut self, model: TtsModel) -> Self {
        self.request.model = model;
        self
    }

    pub fn voice(mut self, voice: Voice) -> Self {
        self.request.voice = voice;
        self
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request.instructions = Some(instructions.into());
        self
    }

    pub fn response_format(mut self, format: AudioFormat) -> Self {
        self.request.response_format = Some(format);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.request.speed = Some(speed);
        self
    }

    pub async fn execute(self) -> Result<SpeechResponse, TtsError> {
        self.client.send_speech_request(self.request).await
    }
}
```

types.rs
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TtsModel {
    #[serde(rename = "tts-1")]
    Tts1,
    #[serde(rename = "tts-1-hd")]
    Tts1Hd,
    #[serde(rename = "gpt-4o-mini-tts")]
    Gpt4oMiniTts,
}

impl TtsModel {
    pub fn as_str(&self) -> &str {
        match self {
            TtsModel::Tts1 => "tts-1",
            TtsModel::Tts1Hd => "tts-1-hd",
            TtsModel::Gpt4oMiniTts => "gpt-4o-mini-tts",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Onyx,
    Nova,
    Sage,
    Shimmer,
    Verse,
}

impl Voice {
    pub fn as_str(&self) -> &str {
        match self {
            Voice::Alloy => "alloy",
            Voice::Ash => "ash",
            Voice::Ballad => "ballad",
            Voice::Coral => "coral",
            Voice::Echo => "echo",
            Voice::Fable => "fable",
            Voice::Onyx => "onyx",
            Voice::Nova => "nova",
            Voice::Sage => "sage",
            Voice::Shimmer => "shimmer",
            Voice::Verse => "verse",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Mp3,
    Opus,
    Aac,
    Flac,
    Wav,
    Pcm,
}

impl AudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Opus => "opus",
            AudioFormat::Aac => "aac",
            AudioFormat::Flac => "flac",
            AudioFormat::Wav => "wav",
            AudioFormat::Pcm => "pcm",
        }
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::Mp3
    }
}

#[derive(Debug, Clone)]
pub struct SpeechRequest {
    pub input: String,
    pub model: TtsModel,
    pub voice: Voice,
    pub instructions: Option<String>,
    pub response_format: Option<AudioFormat>,
    pub speed: Option<f32>,
}

impl Default for SpeechRequest {
    fn default() -> Self {
        Self {
            input: String::new(),
            model: TtsModel::Tts1,
            voice: Voice::Alloy,
            instructions: None,
            response_format: None,
            speed: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpeechRequestJson {
    pub input: String,
    pub model: String,
    pub voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

impl From<SpeechRequest> for SpeechRequestJson {
    fn from(req: SpeechRequest) -> Self {
        Self {
            input: req.input,
            model: req.model.as_str().to_string(),
            voice: req.voice.as_str().to_string(),
            instructions: req.instructions,
            response_format: req.response_format.map(|f| f.as_str().to_string()),
            speed: req.speed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpeechResponse {
    pub audio_data: Vec<u8>,
    pub format: AudioFormat,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub code: Option<String>,
}
```

*ElevenLabs TTS api*

Create speech
POST

https://api.elevenlabs.io
/v1/text-to-speech/:voice_id
POST
/v1/text-to-speech/:voice_id

cURL

curl -X POST "https://api.elevenlabs.io/v1/text-to-speech/JBFqnCBsd6RMkjVDRZzb?output_format=mp3_44100_128" \
     -H "xi-api-key: xi-api-key" \
     -H "Content-Type: application/json" \
     -d '{
  "text": "The first move is what sets everything in motion.",
  "model_id": "eleven_multilingual_v2"
}'
Try it
Converts text into speech using a voice of your choice and returns audio.
Path parameters
voice_id
string
Required
ID of the voice to be used. Use the Get voices endpoint list all the available voices.

Headers
xi-api-key
string
Required
Query parameters
enable_logging
boolean
Optional
Defaults to true
When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.

optimize_streaming_latency
integer or null
Optional
Deprecated
You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values: 0 - default mode (no latency optimizations) 1 - normal latency optimizations (about 50% of possible latency improvement of option 3) 2 - strong latency optimizations (about 75% of possible latency improvement of option 3) 3 - max latency optimizations 4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.

output_format
enum
Optional
Defaults to mp3_44100_128
Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.


Show 19 enum values
Request
This endpoint expects an object.
text
string
Required
The text that will get converted into speech.
model_id
string
Optional
Defaults to eleven_multilingual_v2
Identifier of the model that will be used, you can query them using GET /v1/models. The model needs to have support for text to speech, you can check this using the can_do_text_to_speech property.

language_code
string or null
Optional
Language code (ISO 639-1) used to enforce a language for the model and text normalization. If the model does not support provided language code, an error will be returned.

voice_settings
object or null
Optional
Voice settings overriding stored settings for the given voice. They are applied only on the given request.

Show 5 properties
pronunciation_dictionary_locators
list of objects or null
Optional
A list of pronunciation dictionary locators (id, version_id) to be applied to the text. They will be applied in order. You may have up to 3 locators per request


Show 2 properties
seed
integer or null
Optional
If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed. Must be integer between 0 and 4294967295.
previous_text
string or null
Optional
The text that came before the text of the current request. Can be used to improve the speech's continuity when concatenating together multiple generations or to influence the speech's continuity in the current generation.
next_text
string or null
Optional
The text that comes after the text of the current request. Can be used to improve the speech's continuity when concatenating together multiple generations or to influence the speech's continuity in the current generation.
previous_request_ids
list of strings or null
Optional
A list of request_id of the samples that were generated before this generation. Can be used to improve the speech’s continuity when splitting up a large task into multiple requests. The results will be best when the same model is used across the generations. In case both previous_text and previous_request_ids is send, previous_text will be ignored. A maximum of 3 request_ids can be send.

next_request_ids
list of strings or null
Optional
A list of request_id of the samples that come after this generation. next_request_ids is especially useful for maintaining the speech’s continuity when regenerating a sample that has had some audio quality issues. For example, if you have generated 3 speech clips, and you want to improve clip 2, passing the request id of clip 3 as a next_request_id (and that of clip 1 as a previous_request_id) will help maintain natural flow in the combined speech. The results will be best when the same model is used across the generations. In case both next_text and next_request_ids is send, next_text will be ignored. A maximum of 3 request_ids can be send.

apply_text_normalization
enum
Optional
Defaults to auto
This parameter controls text normalization with three modes: ‘auto’, ‘on’, and ‘off’. When set to ‘auto’, the system will automatically decide whether to apply text normalization (e.g., spelling out numbers). With ‘on’, text normalization will always be applied, while with ‘off’, it will be skipped. For ‘eleven_turbo_v2_5’ and ‘eleven_flash_v2_5’ models, text normalization can only be enabled with Enterprise plans.

Allowed values:
auto
on
off
apply_language_text_normalization
boolean
Optional
Defaults to false
This parameter controls language text normalization. This helps with proper pronunciation of text in some supported languages. WARNING: This parameter can heavily increase the latency of the request. Currently only supported for Japanese.

use_pvc_as_ivc
boolean
Optional
Defaults to false
Deprecated
If true, we won't use PVC version of the voice for the generation but the IVC version. This is a temporary workaround for higher latency in PVC versions.
Response
The generated audio file

*Eleven Labs models*

eleven_v3
eleven_multilingual_v2
eleven_flash_v2_5
eleven_turbo_v2_5

*Eleven labs voices*

      "voice_id": "21m00Tcm4TlvDq8ikWAM",
      "name": "Rachel",
      
      "voice_id": "29vD33N1CtxCmqQRPOHJ",
      "name": "Drew",

      "voice_id": "2EiwWnXFnvU5JabPnv8n",
      "name": "Clyde",
      
      "voice_id": "5Q0t7uMcjvnagumLfvZi",
      "name": "Paul",
      
      "voice_id": "9BWtsMINqrJLrRacOk9x",
      "name": "Aria",
      
      "voice_id": "AZnzlk1XvdvUeBnXmlld",
      "name": "Domi",
      
      "voice_id": "CYw3kZ02Hs0563khs1Fj",
      "name": "Dave",
      
      "voice_id": "CwhRBWXzGAHq8TQ4Fs17",
      "name": "Roger",
      
      "voice_id": "D38z5RcWu1voky8WS1ja",
      "name": "Fin",
      
      "voice_id": "EXAVITQu4vr4xnSDxMaL",
      "name": "Sarah",

*Hyperware HTTP Client API*

```rust
use http::Method;

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum HttpClientError {
    // HTTP errors
    #[error("request could not be deserialized to valid HttpClientRequest")]
    MalformedRequest,
    #[error("http method not supported: {method}")]
    BadMethod { method: String },
    #[error("url could not be parsed: {url}")]
    BadUrl { url: String },
    #[error("http version not supported: {version}")]
    BadVersion { version: String },
    #[error("client failed to build request: {0}")]
    BuildRequestFailed(String),
    #[error("client failed to execute request: {0}")]
    ExecuteRequestFailed(String),

    // WebSocket errors
    #[error("could not open connection to {url}")]
    WsOpenFailed { url: String },
    #[error("sent WebSocket push to unknown channel {channel_id}")]
    WsPushUnknownChannel { channel_id: u32 },
    #[error("WebSocket push failed because message had no blob attached")]
    WsPushNoBlob,
    #[error("WebSocket push failed because message type was Text, but blob was not a valid UTF-8 string")]
    WsPushBadText,
    #[error("failed to close connection {channel_id} because it was not open")]
    WsCloseFailed { channel_id: u32 },
}

/// Make an HTTP request using http-client and await its response.
pub async fn send_request_await_response(
    method: Method,
    url: url::Url,
    headers: Option<HashMap<String, String>>,
    timeout: u64,
    body: Vec<u8>,
) -> std::result::Result<http::Response<Vec<u8>>, HttpClientError>
```

*Prompt*

Create an implementation plan for a Rust library for Hyperware processes to access the ElevenLabs TTS API

Above are docs for the ElevenLabs TTS API and hyperware HTTP client methods. Use the Hyperware HTTP client methods to talk to the ElevenLabs API. Do not use ever use streaming mode

Above is also provided the interface to the already-implemented Hyperware-OpenAI TTS API. Use this as a guide for how the interface to ElevenLabs API should look: ideally the interfaces should be, if not identical, at least similar
