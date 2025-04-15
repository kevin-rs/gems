use crate::chat::Chats;
use crate::embed::Embeddings;
use crate::imagen::Images;
use crate::models::Model;
use crate::models::Models;
use crate::stream::Streaming;
use crate::tokens::Tokens;
use crate::traits::CTrait;
use crate::vision::Visions;
use anyhow::anyhow;
use anyhow::Result;
use reqwest::Client as HttpClient;
use reqwest::Method;
use reqwest::RequestBuilder;
use reqwest::Url;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Client {
    http_client: Arc<HttpClient>,
    api_key: Arc<RwLock<Option<String>>>,
    model: Arc<RwLock<Model>>,
    base_url: String,
}

impl Client {
    pub fn builder() -> CBuilder {
        CBuilder::default()
    }
}

impl CTrait for Client {
    fn set_api_key(&self, api_key: String) {
        let mut key = self.api_key.write().unwrap();
        *key = Some(api_key);
    }

    fn get_api_key(&self) -> Option<String> {
        self.api_key.read().unwrap().clone()
    }

    fn get_model(&self) -> Model {
        self.model.read().unwrap().clone()
    }

    fn set_model(&mut self, model: Model) {
        self.model = Arc::new(RwLock::new(model));
    }

    fn request(&self, method: Method, endpoint: &str) -> Result<RequestBuilder> {
        let api_key = self.get_api_key().ok_or(anyhow!("API key not set"))?;

        let full_url = if endpoint == "models" {
            GEMINI_BASE_URL.to_string()
        } else if endpoint.is_empty() {
            format!("{}/{}", GEMINI_BASE_URL, self.get_model().to_string())
        } else {
            format!(
                "{}/{}:{}",
                GEMINI_BASE_URL,
                self.get_model().to_string(),
                endpoint
            )
        };

        let parsed_url = Url::parse_with_params(&full_url, &[("key", api_key)]).unwrap();

        Ok(self
            .http_client
            .request(method, parsed_url)
            .header("Content-Type", "application/json"))
    }

    fn chat(&self) -> Chats {
        Chats {
            client: self.clone(),
        }
    }
    fn embeddings(&self) -> Embeddings {
        Embeddings {
            client: self.clone(),
        }
    }

    fn tokens(&self) -> Tokens {
        Tokens {
            client: self.clone(),
        }
    }

    fn vision(&self) -> Visions {
        Visions {
            client: self.clone(),
        }
    }

    fn stream(&self) -> Streaming {
        Streaming {
            client: self.clone(),
        }
    }

    fn models(&self) -> Models {
        Models {
            client: self.clone(),
        }
    }

    fn images(&self) -> Images {
        Images {
            client: self.clone(),
        }
    }
}

#[derive(Default)]
pub struct CBuilder {
    model: Option<Model>,
    base_url: Option<String>,
}

impl CBuilder {
    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(Model::from_str(model).unwrap_or_default());
        self
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn build(self) -> Result<Client> {
        let model = self.model.unwrap_or_default();

        Ok(Client {
            http_client: Arc::new(
                HttpClient::builder()
                    .danger_accept_invalid_certs(true)
                    .build()?,
            ),
            api_key: Arc::new(RwLock::new(None)),
            model: Arc::new(RwLock::new(model)),
            base_url: self.base_url.unwrap_or_else(|| GEMINI_BASE_URL.to_string()),
        })
    }
}
