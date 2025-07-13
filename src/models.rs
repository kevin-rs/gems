use crate::responses::ModelInfo;
use crate::responses::ModelsResponse;
use crate::traits::CTrait;
use crate::Client;
use anyhow::{anyhow, Error};
use derive_builder::Builder;
use reqwest::Method;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Model {
    Pro25Preview,
    #[default]
    Flash20,
    Flash20Lite,
    Flash15,
    Flash15_8B,
    Pro15,
    Embedding,
    Imagen3,
    Veo2,
    Tts,
    Flash20Live,
    FlashExpImage,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Model {
    fn to_string(&self) -> String {
        match self {
            Model::Pro25Preview => "gemini-2.5-pro-preview-03-25",
            Model::Flash20 => "gemini-2.0-flash",
            Model::Flash20Lite => "gemini-2.0-flash-lite",
            Model::Flash15 => "gemini-1.5-flash",
            Model::Flash15_8B => "gemini-1.5-flash-8b",
            Model::Pro15 => "gemini-1.5-pro",
            Model::Embedding => "text-embedding-004",
            Model::Imagen3 => "imagen-3.0-generate-002",
            Model::Veo2 => "veo-2.0-generate-001",
            Model::Tts => "gemini-2.5-flash-preview-tts",
            Model::Flash20Live => "gemini-2.0-flash-live-001",
            Model::FlashExpImage => "gemini-2.0-flash-exp-image-generation",
        }
        .to_string()
    }
}

impl FromStr for Model {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gemini-2.5-pro-preview-03-25" => Ok(Model::Pro25Preview),
            "gemini-2.0-flash" => Ok(Model::Flash20),
            "gemini-2.0-flash-lite" => Ok(Model::Flash20Lite),
            "gemini-1.5-flash" => Ok(Model::Flash15),
            "gemini-1.5-flash-8b" => Ok(Model::Flash15_8B),
            "gemini-1.5-pro" => Ok(Model::Pro15),
            "text-embedding-004" => Ok(Model::Embedding),
            "imagen-3.0-generate-002" => Ok(Model::Imagen3),
            "veo-2.0-generate-001" => Ok(Model::Veo2),
            "gemini-2.5-flash-preview-tts" => Ok(Model::Tts),
            "gemini-2.0-flash-live-001" => Ok(Model::Flash20Live),
            "gemini-2.0-flash-exp-image-generation" => Ok(Model::Flash20Live),
            _ => Err(anyhow!("Unknown model: {}", s)),
        }
    }
}
#[derive(Clone)]
pub struct Models {
    pub client: Client,
}

#[derive(Builder, Default, Clone)]
#[builder(setter(into), default)]
pub struct Mod {
    pub model: Model,
}

impl Models {
    pub async fn list(&self) -> anyhow::Result<ModelsResponse> {
        let req = self.client.request(Method::GET, "models")?;

        let res = req.send().await?;
        Ok(res.json().await?)
    }

    pub async fn get(&self, _model: Mod) -> anyhow::Result<ModelInfo> {
        let req = self.client.request(Method::GET, "")?;

        let res = req.send().await?;
        Ok(res.json().await?)
    }
}
