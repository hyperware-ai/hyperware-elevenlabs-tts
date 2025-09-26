use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TtsModel {
    #[serde(rename = "eleven_v3")]
    ElevenV3,
    #[serde(rename = "eleven_multilingual_v2")]
    ElevenMultilingualV2,
    #[serde(rename = "eleven_flash_v2_5")]
    ElevenFlashV25,
    #[serde(rename = "eleven_turbo_v2_5")]
    ElevenTurboV25,
}

impl TtsModel {
    pub fn as_str(&self) -> &str {
        match self {
            TtsModel::ElevenV3 => "eleven_v3",
            TtsModel::ElevenMultilingualV2 => "eleven_multilingual_v2",
            TtsModel::ElevenFlashV25 => "eleven_flash_v2_5",
            TtsModel::ElevenTurboV25 => "eleven_turbo_v2_5",
        }
    }
}

impl Default for TtsModel {
    fn default() -> Self {
        TtsModel::ElevenMultilingualV2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Voice {
    Rachel,
    Drew,
    Clyde,
    Paul,
    Aria,
    Domi,
    Dave,
    Roger,
    Fin,
    Sarah,
}

impl Voice {
    // as of 250926
    pub fn as_voice_id(&self) -> &str {
        match self {
            Voice::Rachel => "21m00Tcm4TlvDq8ikWAM",
            Voice::Drew => "29vD33N1CtxCmqQRPOHJ",
            Voice::Clyde => "2EiwWnXFnvU5JabPnv8n",
            Voice::Paul => "5Q0t7uMcjvnagumLfvZi",
            Voice::Aria => "9BWtsMINqrJLrRacOk9x",
            Voice::Domi => "AZnzlk1XvdvUeBnXmlld",
            Voice::Dave => "CYw3kZ02Hs0563khs1Fj",
            Voice::Roger => "CwhRBWXzGAHq8TQ4Fs17",
            Voice::Fin => "D38z5RcWu1voky8WS1ja",
            Voice::Sarah => "EXAVITQu4vr4xnSDxMaL",
        }
    }
}

impl Default for Voice {
    fn default() -> Self {
        Voice::Rachel
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    #[serde(rename = "mp3_22050_32")]
    Mp3_22050_32,
    #[serde(rename = "mp3_44100_32")]
    Mp3_44100_32,
    #[serde(rename = "mp3_44100_64")]
    Mp3_44100_64,
    #[serde(rename = "mp3_44100_96")]
    Mp3_44100_96,
    #[serde(rename = "mp3_44100_128")]
    Mp3_44100_128,
    #[serde(rename = "mp3_44100_192")]
    Mp3_44100_192,
    #[serde(rename = "pcm_16000")]
    Pcm16000,
    #[serde(rename = "pcm_22050")]
    Pcm22050,
    #[serde(rename = "pcm_24000")]
    Pcm24000,
    #[serde(rename = "pcm_44100")]
    Pcm44100,
    #[serde(rename = "ulaw_8000")]
    Ulaw8000,
}

impl AudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            AudioFormat::Mp3_22050_32 => "mp3_22050_32",
            AudioFormat::Mp3_44100_32 => "mp3_44100_32",
            AudioFormat::Mp3_44100_64 => "mp3_44100_64",
            AudioFormat::Mp3_44100_96 => "mp3_44100_96",
            AudioFormat::Mp3_44100_128 => "mp3_44100_128",
            AudioFormat::Mp3_44100_192 => "mp3_44100_192",
            AudioFormat::Pcm16000 => "pcm_16000",
            AudioFormat::Pcm22050 => "pcm_22050",
            AudioFormat::Pcm24000 => "pcm_24000",
            AudioFormat::Pcm44100 => "pcm_44100",
            AudioFormat::Ulaw8000 => "ulaw_8000",
        }
    }
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::Mp3_44100_128
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            stability: None,
            similarity_boost: None,
            style: None,
            use_speaker_boost: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextNormalization {
    Auto,
    On,
    Off,
}

impl Default for TextNormalization {
    fn default() -> Self {
        TextNormalization::Auto
    }
}

#[derive(Debug, Clone)]
pub struct SpeechRequest {
    pub text: String,
    pub model: TtsModel,
    pub voice: Voice,
    pub voice_settings: Option<VoiceSettings>,
    pub output_format: Option<AudioFormat>,
    pub language_code: Option<String>,
    pub seed: Option<u32>,
    pub previous_text: Option<String>,
    pub next_text: Option<String>,
    pub previous_request_ids: Option<Vec<String>>,
    pub next_request_ids: Option<Vec<String>>,
    pub apply_text_normalization: Option<TextNormalization>,
    pub apply_language_text_normalization: Option<bool>,
}

impl Default for SpeechRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            model: TtsModel::default(),
            voice: Voice::default(),
            voice_settings: None,
            output_format: None,
            language_code: None,
            seed: None,
            previous_text: None,
            next_text: None,
            previous_request_ids: None,
            next_request_ids: None,
            apply_text_normalization: None,
            apply_language_text_normalization: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SpeechRequestJson {
    pub text: String,
    pub model_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_request_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_request_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<TextNormalization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_language_text_normalization: Option<bool>,
}

impl From<SpeechRequest> for SpeechRequestJson {
    fn from(req: SpeechRequest) -> Self {
        Self {
            text: req.text,
            model_id: req.model.as_str().to_string(),
            language_code: req.language_code,
            voice_settings: req.voice_settings,
            seed: req.seed,
            previous_text: req.previous_text,
            next_text: req.next_text,
            previous_request_ids: req.previous_request_ids,
            next_request_ids: req.next_request_ids,
            apply_text_normalization: req.apply_text_normalization,
            apply_language_text_normalization: req.apply_language_text_normalization,
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
