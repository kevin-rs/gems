use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::Content;
use crate::requests::GeminiEmbedRequest;
use crate::requests::GeminiEmbedRequests;
use crate::responses::BatchEmbedContentsResponse;
use crate::responses::EmbedContentResponse;
use crate::traits::CTrait;
use anyhow::Result;
use derive_builder::Builder;
use reqwest::Method;

#[derive(Builder, Default, Clone)]
#[builder(setter(into), default)]
pub struct Embedding {
    pub model: Model,
    pub input: Message,
}

#[derive(Builder, Default, Clone)]
#[builder(setter(into), default)]
pub struct BatchEmbedding {
    pub model: Model,
    pub input: Vec<Message>,
}

#[derive(Clone)]
pub struct Embeddings {
    pub client: Client,
}

impl Embeddings {
    pub async fn create(&self, params: Embedding) -> Result<EmbedContentResponse> {
        let request_body = GeminiEmbedRequest {
            model: format!("models/{}", params.model.to_string()),
            content: Content {
                parts: vec![params.input.to_part()],
            },
        };

        let req = self
            .client
            .request(Method::POST, "embedContent")?
            .json(&request_body);

        let res = req.send().await?;
        Ok(res.json().await?)
    }

    pub async fn batch(&self, params: BatchEmbedding) -> Result<BatchEmbedContentsResponse> {
        let requests = params
            .input
            .into_iter()
            .map(|message| GeminiEmbedRequest {
                model: format!("models/{}", params.model.to_string()),
                content: Content {
                    parts: vec![message.to_part()],
                },
            })
            .collect::<Vec<GeminiEmbedRequest>>();

        let request_body = GeminiEmbedRequests { requests };

        let req = self
            .client
            .request(Method::POST, "batchEmbedContents")?
            .json(&request_body);

        let res = req.send().await?;
        Ok(res.json().await?)
    }
}
