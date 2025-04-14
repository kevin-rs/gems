use crate::messages::Message;
use crate::models::Model;
use crate::requests::Content;
use crate::requests::GeminiRequest;
use crate::traits::CTrait;
use crate::Client;
use anyhow::Result;
use derive_builder::Builder;
use reqwest::Method;
use serde_json::Value;

#[derive(Clone)]
pub struct Tokens {
    pub client: Client,
}

#[derive(Builder, Default, Clone)]
#[builder(setter(into), default)]
pub struct Token {
    model: Model,
    input: Message,
}

impl Tokens {
    pub async fn count(&self, params: Token) -> Result<usize> {
        let request_body = GeminiRequest {
            model: params.model.to_string(),
            contents: vec![Content {
                parts: vec![params.input.to_part()],
            }],
            config: None,
        };

        let req = self
            .client
            .request(Method::POST, "countTokens")?
            .json(&request_body);

        let res = req.send().await?;
        let json: Value = res.json().await?;

        Ok(json["totalTokens"].as_u64().unwrap_or(0) as usize)
    }
}
