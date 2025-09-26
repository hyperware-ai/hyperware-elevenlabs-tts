# ElevenLabs TTS Library Implementation Plan

## Overview
Create a Rust library for Hyperware processes to access the ElevenLabs Text-to-Speech API, following the established patterns from the Hyperware OpenAI TTS implementation. The library will provide a clean, type-safe interface for converting text to speech using ElevenLabs' advanced voice synthesis models.

## Key Design Principles
1. **API Compatibility**: Maintain similar interface patterns to the existing OpenAI TTS implementation for consistency
2. **Type Safety**: Use Rust's type system to ensure compile-time validation of API parameters
3. **Error Handling**: Comprehensive error types for all failure modes
4. **Non-Streaming**: Focus on request/response pattern without streaming support (as specified)
5. **Builder Pattern**: Use builder pattern for constructing requests with sensible defaults

## Module Structure

### 1. `types.rs` - Type Definitions

#### Core Types to Implement:

```rust
// Voice IDs mapping to ElevenLabs voices
pub enum Voice {
    Rachel,    // 21m00Tcm4TlvDq8ikWAM
    Drew,      // 29vD33N1CtxCmqQRPOHJ
    Clyde,     // 2EiwWnXFnvU5JabPnv8n
    Paul,      // 5Q0t7uMcjvnagumLfvZi
    Aria,      // 9BWtsMINqrJLrRacOk9x
    Domi,      // AZnzlk1XvdvUeBnXmlld
    Dave,      // CYw3kZ02Hs0563khs1Fj
    Roger,     // CwhRBWXzGAHq8TQ4Fs17
    Fin,       // D38z5RcWu1voky8WS1ja
    Sarah,     // EXAVITQu4vr4xnSDxMaL
}

// Model IDs for ElevenLabs
pub enum TtsModel {
    ElevenV3,
    ElevenMultilingualV2,
    ElevenFlashV2_5,
    ElevenTurboV2_5,
}

// Audio formats supported by ElevenLabs
pub enum AudioFormat {
    Mp3_22050_32,
    Mp3_44100_32,
    Mp3_44100_64,
    Mp3_44100_96,
    Mp3_44100_128,
    Mp3_44100_192,
    Pcm_16000,
    Pcm_22050,
    Pcm_24000,
    Pcm_44100,
    Ulaw_8000,
}

// Voice settings for fine-tuning
pub struct VoiceSettings {
    stability: Option<f32>,        // 0.0 to 1.0
    similarity_boost: Option<f32>, // 0.0 to 1.0
    style: Option<f32>,            // 0.0 to 1.0
    use_speaker_boost: Option<bool>,
}

// Text normalization modes
pub enum TextNormalization {
    Auto,
    On,
    Off,
}

// Main request structure
pub struct SpeechRequest {
    text: String,
    voice_id: String,  // Actual voice ID
    model_id: String,  // Actual model ID
    voice_settings: Option<VoiceSettings>,
    output_format: Option<AudioFormat>,
    language_code: Option<String>,
    seed: Option<u32>,
    previous_text: Option<String>,
    next_text: Option<String>,
    apply_text_normalization: Option<TextNormalization>,
}

// Response structure
pub struct SpeechResponse {
    audio_data: Vec<u8>,
    format: AudioFormat,
}
```

### 2. `client.rs` - Client Implementation

#### Key Components:

1. **SpeechClient Structure**
   - API key management
   - Base URL configuration (default: https://api.elevenlabs.io)
   - Request timeout settings
   - HTTP client integration using Hyperware's `send_request_await_response`

2. **SpeechRequestBuilder**
   - Fluent interface for building requests
   - Methods matching OpenAI pattern:
     - `input()` → `text()` (renamed for ElevenLabs terminology)
     - `voice()` (enum to voice_id conversion)
     - `model()` (enum to model_id conversion)
     - `voice_settings()` (new, ElevenLabs-specific)
     - `response_format()` → `output_format()`
     - `language_code()` (new, for language enforcement)
     - `seed()` (for deterministic generation)
     - `previous_text()` / `next_text()` (for context)
     - `execute()` (sends request and returns response)

3. **Request Processing Flow**
   ```
   1. Validate input parameters
   2. Convert enums to API string values
   3. Build JSON request body
   4. Add authentication headers (xi-api-key)
   5. Send POST to /v1/text-to-speech/{voice_id}
   6. Parse response or error
   7. Return SpeechResponse with audio data
   ```

### 3. `error.rs` - Error Handling

```rust
pub enum TtsError {
    // Input validation errors
    MissingInput,
    InputTooLong(usize),
    InvalidVoiceSettings { field: String, value: f32 },
    
    // API errors
    MissingApiKey,
    ApiError { status: u16, message: String },
    
    // HTTP/Network errors
    HttpClient(HttpClientError),
    
    // Serialization errors
    SerializationError(String),
    DeserializationError(String),
}
```

### 4. `lib.rs` - Public API

Export structure:
```rust
pub mod client;
pub mod error;
pub mod types;

pub use client::{SpeechClient, SpeechRequestBuilder};
pub use error::TtsError;
pub use types::{
    AudioFormat, SpeechRequest, SpeechResponse,
    TtsModel, Voice, VoiceSettings, TextNormalization
};
```

## Implementation Steps

### Phase 1: Core Types and Structures
1. Create `Cargo.toml` with dependencies:
   - `serde` with derive features
   - `serde_json`
   - `http`
   - `url`
   - `hyperware_process_lib` (for HTTP client)

2. Implement `types.rs`:
   - Define all enums with proper serde attributes
   - Implement conversion methods (to_string/as_str)
   - Create request/response structures

### Phase 2: Client Implementation
1. Implement `client.rs`:
   - Create SpeechClient with configuration methods
   - Implement SpeechRequestBuilder with fluent interface
   - Add request validation logic
   - Implement HTTP request construction
   - Handle response parsing

### Phase 3: Error Handling
1. Implement `error.rs`:
   - Define comprehensive error enum
   - Implement Display and Error traits
   - Add conversion from HTTP client errors

### Phase 4: Integration and Testing
1. Create `lib.rs` with public exports
2. Add integration tests
3. Create usage examples

## API Mapping (OpenAI → ElevenLabs)

| OpenAI Field | ElevenLabs Field | Notes |
|--------------|------------------|-------|
| `input` | `text` | Direct mapping |
| `model` | `model_id` | Different model names |
| `voice` | `voice_id` | Need to map enum to IDs |
| `response_format` | `output_format` | Different format options |
| `speed` | N/A | Not supported in ElevenLabs |
| `instructions` | N/A | Not supported in ElevenLabs |
| N/A | `voice_settings` | ElevenLabs-specific |
| N/A | `language_code` | ElevenLabs-specific |
| N/A | `seed` | For deterministic output |
| N/A | `previous_text`/`next_text` | Context for continuity |

## Usage Example

```rust
use hyperware_elevenlabs_tts::{SpeechClient, Voice, TtsModel, AudioFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SpeechClient::new("your-api-key")
        .with_timeout(30000);
    
    let response = client
        .synthesize()
        .text("Hello, this is a test of ElevenLabs TTS")
        .voice(Voice::Sarah)
        .model(TtsModel::ElevenMultilingualV2)
        .output_format(AudioFormat::Mp3_44100_128)
        .execute()
        .await?;
    
    // Save audio data to file
    std::fs::write("output.mp3", response.audio_data)?;
    
    Ok(())
}
```

## Key Differences from OpenAI Implementation

1. **Voice Selection**: Uses voice IDs instead of simple names
2. **Additional Parameters**: Voice settings, language code, seed for determinism
3. **Context Support**: Previous/next text for better continuity
4. **No Speed Control**: ElevenLabs doesn't support speed adjustment
5. **Different Models**: ElevenLabs-specific model names
6. **Authentication**: Uses `xi-api-key` header instead of Bearer token

## Error Handling Strategy

1. **Validation Errors**: Check before sending request
2. **API Errors**: Parse ElevenLabs error responses
3. **Network Errors**: Wrap Hyperware HTTP client errors
4. **Graceful Degradation**: Provide sensible defaults where possible

## Future Enhancements (Out of Scope)

- Streaming support (explicitly excluded)
- Voice cloning endpoints
- Voice library management
- Pronunciation dictionary support
- Request stitching with request IDs