use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::{
    Content, Part, PrebuiltVoiceConfig, SpeechConfig, TtsGenerationConfig, TtsRequest, VoiceConfig,
};
use crate::responses::{ErrorWrapper, Part as ResPart, TtsResponse};
use crate::traits::CTrait;
use anyhow::{anyhow, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use derive_builder::Builder;
use reqwest::Method;

#[derive(Clone)]
pub struct Tts {
    pub client: Client,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct TtsGen {
    pub model: Model,
    pub input: Message,
    #[builder(setter(into, strip_option), default)]
    pub system: Option<Vec<Message>>,
    #[builder(default = "String::from(\"Kore\")")]
    pub voice: String,
}

impl Tts {
    pub async fn generate(&self, params: TtsGen) -> Result<Vec<u8>> {
        let content = Content {
            parts: vec![params.input.to_part()],
        };

        let prompt_text = content
            .parts
            .iter()
            .find_map(|part| {
                if let Part::Text { text } = part {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("Prompt must contain text"))?;

        let system_instruction = params.system.as_ref().map(|messages| Content {
            parts: messages.iter().map(|msg| msg.to_part()).collect(),
        });

        let request_body = TtsRequest {
            model: params.model.to_string(),
            contents: vec![Content {
                parts: vec![Part::Text {
                    text: prompt_text.clone(),
                }],
            }],
            generation_config: TtsGenerationConfig {
                response_modalities: vec!["AUDIO".into()],
                speech_config: SpeechConfig {
                    voice_config: VoiceConfig {
                        prebuilt_voice_config: PrebuiltVoiceConfig {
                            voice_name: params.voice.clone(),
                        },
                    },
                },
            },
            system_instruction,
        };

        let req = self
            .client
            .request(Method::POST, "generateContent")?
            .json(&request_body);

        let res = req.send().await?;

        if !res.status().is_success() {
            let body = res.text().await?;
            let err = match serde_json::from_str::<ErrorWrapper>(&body) {
                Ok(err_struct) => err_struct.error.message,
                Err(_) => body,
            };
            return Err(anyhow!("TTS generation failed: {}", err));
        }

        let json: TtsResponse = res.json().await?;
        let audio_part = json
            .candidates
            .and_then(|mut c| c.pop())
            .and_then(|c| {
                c.content.parts.into_iter().find_map(|part| match part {
                    ResPart::Image { inline_data } => Some(inline_data.data),
                    _ => None,
                })
            })
            .ok_or_else(|| anyhow!("No audio found in response"))?;

        STANDARD
            .decode(&audio_part)
            .map_err(|e| anyhow!("Failed to decode audio: {}", e))
    }
}
