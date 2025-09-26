# Hyperware ElevenLabs TTS

A Rust library for Hyperware processes to access the ElevenLabs Text-to-Speech API.

## Features

- Type-safe interface for ElevenLabs TTS API
- Builder pattern for constructing requests
- Support for all ElevenLabs voices and models
- Comprehensive voice settings control
- Language-specific text normalization
- Context support with previous/next text
- Deterministic generation with seed support

## Usage

```rust
use hyperware_elevenlabs_tts::{SpeechClient, Voice, TtsModel, AudioFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SpeechClient::new("your-xi-api-key")
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

## Advanced Usage with Voice Settings

```rust
use hyperware_elevenlabs_tts::{SpeechClient, Voice, TtsModel, VoiceSettings};

let response = client
    .synthesize()
    .text("Advanced voice synthesis")
    .voice(Voice::Rachel)
    .model(TtsModel::ElevenTurboV25)
    .stability(0.75)           // Voice consistency
    .similarity_boost(0.85)     // Voice clarity
    .style(0.5)                // Speaking style
    .use_speaker_boost(true)    // Enhanced speaker characteristics
    .language_code("en")
    .seed(42)                   // For reproducible output
    .execute()
    .await?;
```

## Available Voices

- `Rachel` - Natural, conversational female voice
- `Drew` - Deep male voice
- `Clyde` - Mature male voice
- `Paul` - Clear male voice
- `Aria` - Young female voice
- `Domi` - Energetic female voice
- `Dave` - British male voice
- `Roger` - Mature male voice
- `Fin` - Irish male voice
- `Sarah` - American female voice

## Available Models

- `ElevenV3` - Latest generation model
- `ElevenMultilingualV2` - Multilingual support (default)
- `ElevenFlashV25` - Fast, low-latency model
- `ElevenTurboV25` - Optimized for speed

## Audio Formats

- MP3: `Mp3_22050_32`, `Mp3_44100_32`, `Mp3_44100_64`, `Mp3_44100_96`, `Mp3_44100_128`, `Mp3_44100_192`
- PCM: `Pcm16000`, `Pcm22050`, `Pcm24000`, `Pcm44100`
- μ-law: `Ulaw8000` (for Twilio)

## API Compatibility

This library follows a similar interface pattern to the Hyperware OpenAI TTS implementation for consistency:

- `input()` and `text()` methods are interchangeable
- `response_format()` and `output_format()` methods are interchangeable
- Builder pattern for constructing requests
- Comprehensive error handling

## Dependencies

When integrating with actual Hyperware infrastructure, update `Cargo.toml`:

```toml
[dependencies]
hyperware_process_lib = { path = "../hyperware_process_lib" }
```

And update the imports in `client.rs` and `error.rs` to use the actual HTTP client.

## License

Internal Hyperware library - see organization license.