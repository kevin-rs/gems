use crate::messages::Content;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::Content as ReqContent;
use crate::requests::GeminiRequest;
use crate::requests::ImageContent;
use crate::requests::Part;
use crate::responses::GeminiResponse;
use crate::traits::CTrait;
use crate::Client;
use anyhow::anyhow;
use anyhow::Result;
use derive_builder::Builder;
use reqwest::Method;

#[derive(Clone)]
pub struct Visions {
    pub client: Client,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct Vision {
    pub input: Message,
    pub image: Message,
    #[builder(setter(into, strip_option), default)]
    pub system: Option<Vec<Message>>,
}

impl Visions {
    pub async fn generate(&self, params: Vision) -> Result<String> {
        let input_part = params.input.to_part();

        let system_instruction = params.system.as_ref().map(|messages| ReqContent {
            parts: messages.iter().map(|msg| msg.to_part()).collect(),
        });

        let image_data = match &params.image {
            Message::Tool { content } => content.clone(),
            Message::User { content, .. }
            | Message::System { content, .. }
            | Message::Developer { content, .. } => match content {
                Content::Text(data) => data.clone(),
            },
        };

        let image_part = Part::image(Some(ImageContent {
            mime_type: "image/jpeg".to_string(),
            data: image_data,
        }));

        let request_body = GeminiRequest {
            model: Model::Pro25Preview.to_string(),
            contents: vec![crate::requests::Content {
                parts: vec![input_part, image_part],
            }],
            system_instruction,
            config: None,
        };

        let req = self
            .client
            .request(Method::POST, "generateContent")?
            .json(&request_body);

        let res = req.send().await?;
        let json: GeminiResponse = res.json().await?;

        let candidates = json
            .candidates
            .ok_or_else(|| anyhow!("Missing candidates"))?;

        match &candidates[0].content.parts[0] {
            Part::Text { text } => Ok(text.clone()),
            _ => Err(anyhow!("Expected text response")),
        }
    }
}
