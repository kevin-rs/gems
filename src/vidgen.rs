use crate::client::Client;
use crate::messages::Message;
use crate::models::Model;
use crate::requests::{Content, Part, VideoGenRequest, VideoParameters, VideoPrompt};
use crate::responses::{ErrorWrapper, OperationStatus, VideoGenResponse};
use crate::traits::CTrait;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use derive_builder::Builder;
use reqwest::Method;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct Videos {
    pub client: Client,
}

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct VideoGen {
    pub model: Model,
    pub input: Message,
    #[builder(setter(into, strip_option), default)]
    pub system: Option<Vec<Message>>,
}

impl Videos {
    pub async fn generate(&self, params: VideoGen) -> Result<Vec<u8>> {
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

        let request_body = VideoGenRequest {
            instances: vec![VideoPrompt {
                prompt: prompt_text,
            }],
            parameters: VideoParameters {
                aspect_ratio: "16:9".into(),
                person_generation: "dont_allow".into(),
            },
        };

        let req = self
            .client
            .request(Method::POST, "predictLongRunning")?
            .json(&request_body);

        let res = req.send().await?;
        let status = res.status();

        if !status.is_success() {
            let err_body = res.text().await?;
            let err_message = match serde_json::from_str::<ErrorWrapper>(&err_body) {
                Ok(wrapper) => wrapper.error.message,
                Err(_) => err_body.clone(),
            };
            return Err(anyhow!(
                "API returned error (status {}): {}",
                status,
                err_message
            ));
        }

        let initial: VideoGenResponse = res.json().await?;
        let op_name = initial
            .name
            .ok_or_else(|| anyhow!("Missing operation name"))?;

        for _ in 0..30 {
            let poll_req = self.client.request(Method::GET, &op_name)?;
            let poll_res = poll_req.send().await?;
            let status: OperationStatus = poll_res.json().await?;

            if status.done.unwrap_or(false) {
                if let Some(response) = status.response {
                    let base64_video = response.output.video.base64_data;
                    return STANDARD
                        .decode(&base64_video)
                        .map_err(|e| anyhow!("Failed to decode video: {}", e));
                } else if let Some(error) = status.error {
                    return Err(anyhow!("Video generation failed: {}", error.message));
                } else {
                    return Err(anyhow!("Operation completed but no response found"));
                }
            }

            sleep(Duration::from_secs(5)).await;
        }

        Err(anyhow!("Timed out waiting for video generation"))
    }
}
