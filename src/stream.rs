use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::Content;
use crate::requests::GeminiRequest;
use crate::traits::CTrait;
use anyhow::Result;
use derive_builder::Builder;
use reqwest::Method;
use reqwest::Response;

#[derive(Clone)]
pub struct Streaming {
    pub client: Client,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct Stream {
    pub model: Model,
    pub input: Message,
}

impl Streaming {
    pub async fn generate(&self, params: Stream) -> Result<Response> {
        let request_body = GeminiRequest {
            model: params.model.to_string(),
            contents: vec![Content {
                parts: vec![params.input.to_part()],
            }],
            config: None,
        };

        let req = self
            .client
            .request(Method::POST, "streamGenerateContent")?
            .json(&request_body);

        let res = req.send().await?;
        Ok(res)
    }
}
