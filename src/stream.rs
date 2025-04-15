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
    #[builder(setter(into, strip_option), default)]
    pub system: Option<Vec<Message>>,
}

impl Streaming {
    pub async fn generate(&self, params: Stream) -> Result<Response> {
        let system_instruction = params.system.as_ref().map(|messages| Content {
            parts: messages.iter().map(|msg| msg.to_part()).collect(),
        });

        let request_body = GeminiRequest {
            model: params.model.to_string(),
            contents: vec![Content {
                parts: vec![params.input.to_part()],
            }],
            system_instruction,
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
