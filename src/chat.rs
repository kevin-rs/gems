use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::Content;
use crate::requests::GeminiRequest;
use crate::requests::Part;
use crate::responses::GeminiResponse;
use crate::traits::CTrait;
use anyhow::anyhow;
use anyhow::Result;
use derive_builder::Builder;
use reqwest::Method;

#[derive(Builder, Default, Clone)]
#[builder(setter(into), default)]
pub struct Chat {
    pub model: Model,
    pub messages: Vec<Message>,
}

#[derive(Clone)]
pub struct Chats {
    pub client: Client,
}

impl Chats {
    pub async fn generate(&self, params: Chat) -> Result<String> {
        let content = Content {
            parts: params.messages.iter().map(|msg| msg.to_part()).collect(),
        };

        let request_body = GeminiRequest {
            model: params.model.to_string(),
            contents: vec![content],
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
