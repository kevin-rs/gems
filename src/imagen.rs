use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::GenerationConfig;
use crate::requests::{Content, GeminiRequest};
use crate::responses::ImagenResponse;
use crate::traits::CTrait;
use crate::utils::extract_image_or_text;
use anyhow::{anyhow, Result};
use derive_builder::Builder;
use reqwest::Method;

#[derive(Clone)]
pub struct Images {
    pub client: Client,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct ImageGen {
    pub model: Model,
    pub input: Message,
    #[builder(setter(into, strip_option), default)]
    pub system: Option<Vec<Message>>,
}

impl Images {
    pub async fn generate(&self, params: ImageGen) -> Result<Vec<u8>> {
        let content = Content {
            parts: vec![params.input.to_part()],
        };

        let system_instruction = params.system.as_ref().map(|messages| Content {
            parts: messages.iter().map(|msg| msg.to_part()).collect(),
        });

        let request_body = GeminiRequest {
            model: params.model.to_string(),
            contents: vec![content],
            system_instruction,
            config: Some(GenerationConfig {
                response_modalities: vec!["Text".into(), "Image".into()],
            }),
        };

        let req = self
            .client
            .request(Method::POST, "generateContent")?
            .json(&request_body);

        let res = req.send().await?;
        let json: ImagenResponse = res.json().await?;

        let parts = json
            .candidates
            .ok_or_else(|| anyhow!("Missing candidates"))?
            .first()
            .ok_or_else(|| anyhow!("No candidate response"))?
            .content
            .parts
            .clone();

        extract_image_or_text(&parts)
    }
}
