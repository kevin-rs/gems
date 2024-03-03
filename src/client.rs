use crate::requests::Content;
use crate::requests::GeminiEmbedRequest;
use crate::requests::GeminiEmbedRequests;
use crate::requests::GeminiRequest;
use crate::requests::Part;
use crate::responses::BatchEmbedContentsResponse;
use crate::responses::EmbedContentResponse;
use crate::responses::GeminiResponse;
use crate::responses::ModelInfo;
use crate::responses::ModelsResponse;
use crate::utils::{extract_text_from_partial_json, type_with_cursor_effect};
use futures_util::StreamExt;
use reqwest::header;
use reqwest::Client as ReqClient;
use reqwest::Url;
use serde_json::Value;
use std::error::Error;

const GEMINI_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent";

/// Gemini API client structure.
#[derive(Debug, Clone)]
pub struct Client {
    /// Reqwest client instance.
    pub client: ReqClient,

    /// API key for authentication.
    pub api_key: String,

    /// Model to be used.
    pub model: String,

    /// API URL for Gemini.
    pub api_url: Url,
}

impl Client {
    /// Creates a new instance of the Gemini Client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A static string representing the API key for authentication.
    /// * `model` - A static string representing the model to be used.
    ///
    /// # Returns
    ///
    /// A new instance of the Gemini Client.
    ///
    /// # Panics
    ///
    /// Panics if there is an issue parsing the Gemini API URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// let client = Client::new("your_api_key", "your_model");
    /// ```
    pub fn new(api_key: &str, model: &str) -> Self {
        let api_url = Url::parse(&GEMINI_API_URL.replace("gemini-pro", model)).unwrap();
        Self {
            client: ReqClient::new(),
            api_key: api_key.to_owned(),
            model: model.to_owned(),
            api_url,
        }
    }

    /// Generates content using the Gemini API.
    ///
    /// # Arguments
    ///
    /// * `text` - A static string representing the input text for content generation.
    ///
    /// # Returns
    ///
    /// A Result containing the generated content as a string or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.generate_content("input_text").await;
    ///     match result {
    ///         Ok(content) => println!("Generated Content: {}", content),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn generate_content(
        &mut self,
        text: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request_body = GeminiRequest {
            model: self.model.to_string(),
            contents: vec![Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            }],
        };

        self.api_url
            .set_query(Some(&format!("key={}", self.api_key)));

        let response = self
            .client
            .post(self.api_url.clone())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let json: GeminiResponse = response.json().await?;
        let resp = json.candidates.ok_or("")?;
        Ok(resp[0].content.parts[0].text.clone())
    }

    /// Streams generated content using the Gemini API and prints it with a delay effect.
    ///
    /// # Arguments
    ///
    /// * `text` - A static string representing the input text for content generation.
    /// * `suppress` - A boolean to decide whether or not to print the content being generated
    ///
    /// # Returns
    ///
    /// A Result indicating success or a Box<dyn Error> on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.stream_generate_content("input_text", false).await;
    ///     match result {
    ///         Ok(_) => println!("Content streaming completed."),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn stream_generate_content(&mut self, text: &str, suppress: bool) -> Result<String, Box<dyn Error>> {
        let request_body = GeminiRequest {
            model: self.model.to_string(),
            contents: vec![Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            }],
        };

        let count_tokens_url = self
            .api_url
            .join(&format!(
                "/v1beta/models/{}:streamGenerateContent",
                self.model
            ))
            .unwrap();
        let count_tokens_url_with_key = count_tokens_url
            .clone()
            .join(&format!("?key={}", self.api_key))
            .unwrap();

        let response = self
            .client
            .post(count_tokens_url_with_key.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let mut stream = response.bytes_stream();
        let delay = 5;
        let mut message: String = Default::default();
        while let Some(mut chunk) = stream.next().await {
            if let Ok(parsed_json) = std::str::from_utf8(chunk.as_mut().unwrap()) {
                if let Some(text_value) = extract_text_from_partial_json(parsed_json) {
                    let lines: Vec<&str> = text_value
                        .split("\\n")
                        .flat_map(|s| s.split('\n'))
                        .collect();

                    for line in lines {
                        message.push_str(&line.replace('\\', ""));
                        if !line.is_empty() && !suppress {
                            type_with_cursor_effect(&line.replace('\\', ""), delay);
                        } else {
                            println!("\n");
                        }
                    }
                }
            } else {
                eprintln!("Failed to parse chunk: {:?}", chunk.as_ref().unwrap());
            }
        }

        println!();

        Ok(message)
    }

    /// Counts the number of tokens in the provided text using the Gemini API.
    ///
    /// # Arguments
    ///
    /// * `text` - A static string representing the input text for token counting.
    ///
    /// # Returns
    ///
    /// A Result containing the count of tokens as usize or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.count_tokens("input_text").await;
    ///     match result {
    ///         Ok(count) => println!("Token Count: {}", count),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn count_tokens(&mut self, text: &str) -> Result<usize, reqwest::Error> {
        let request_body = GeminiRequest {
            model: self.model.to_string(),
            contents: vec![Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            }],
        };

        let count_tokens_url = self
            .api_url
            .join(&format!("/v1beta/models/{}:countTokens", self.model))
            .unwrap();
        let count_tokens_url_with_key = count_tokens_url
            .clone()
            .join(&format!("?key={}", self.api_key))
            .unwrap();

        let response = self
            .client
            .post(count_tokens_url_with_key.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let tokens_count: Value = response.json().await?;

        let count = tokens_count["totalTokens"].as_u64().unwrap_or(0) as usize;

        Ok(count)
    }

    /// Retrieves information about the specified model using the Gemini API.
    ///
    /// # Returns
    ///
    /// A Result containing model information or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.get_model_info().await;
    ///     match result {
    ///         Ok(info) => println!("Model Info: {:?}", info),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn get_model_info(&mut self) -> Result<ModelInfo, reqwest::Error> {
        let count_tokens_url = self
            .api_url
            .join(&format!("/v1beta/models/{}", self.model))
            .unwrap();
        let count_tokens_url_with_key = count_tokens_url
            .clone()
            .join(&format!("?key={}", self.api_key))
            .unwrap();

        let response = self
            .client
            .get(count_tokens_url_with_key.as_str())
            .send()
            .await?;

        let model_info: ModelInfo = response.json().await?;

        Ok(model_info)
    }

    /// Embeds content using the Gemini API.
    ///
    /// # Arguments
    ///
    /// * `text` - A static string representing the input text for content embedding.
    ///
    /// # Returns
    ///
    /// A Result containing the embedded content response or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let result = client.embed_content("input_text").await;
    ///     match result {
    ///         Ok(response) => println!("Embedded Content: {:?}", response),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn embed_content(
        &mut self,
        text: &str,
    ) -> Result<EmbedContentResponse, reqwest::Error> {
        let request_body = GeminiEmbedRequest {
            model: self.model.to_string(),
            content: Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            },
        };

        let embed_content_url = self
            .api_url
            .join(&format!("/v1beta/models/{}:embedContent", self.model))
            .unwrap();
        let embed_content_url_with_key = embed_content_url
            .clone()
            .join(&format!("?key={}", self.api_key))
            .unwrap();

        let response = self
            .client
            .post(embed_content_url_with_key.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let json: EmbedContentResponse = response.json().await?;
        Ok(json)
    }

    /// Batch embeds multiple contents using the Gemini API.
    ///
    /// # Arguments
    ///
    /// * `texts` - A vector of static strings representing the input texts for batch embedding.
    ///
    /// # Returns
    ///
    /// A Result containing the batch embedded contents response or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::new("your_api_key", "your_model");
    ///     let texts = vec!["text1", "text2", "text3"];
    ///     let result = client.batch_embed_contents(texts).await;
    ///     match result {
    ///         Ok(response) => println!("Batch Embedded Contents: {:?}", response),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn batch_embed_contents(
        &mut self,
        texts: Vec<&str>,
    ) -> Result<BatchEmbedContentsResponse, reqwest::Error> {
        let requests = texts
            .into_iter()
            .map(|text| GeminiEmbedRequest {
                model: "models/".to_owned() + &self.model,
                content: Content {
                    parts: vec![Part {
                        text: text.to_string(),
                    }],
                },
            })
            .collect();

        let request_body = GeminiEmbedRequests { requests };

        let batch_embed_contents_url = self
            .api_url
            .join(&format!("/v1beta/models/{}:batchEmbedContents", self.model))
            .unwrap();
        let batch_embed_contents_url_with_key = batch_embed_contents_url
            .clone()
            .join(&format!("?key={}", self.api_key))
            .unwrap();

        let response = self
            .client
            .post(batch_embed_contents_url_with_key.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        let json: BatchEmbedContentsResponse = response.json().await?;
        Ok(json)
    }

    /// Retrieves a list of available models from the Gemini API.
    ///
    /// # Returns
    ///
    /// A Result containing the list of models or a reqwest::Error on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use gems::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("your_api_key", "your_model");
    ///     let result = client.list_models().await;
    ///     match result {
    ///         Ok(models) => println!("Available Models: {:?}", models),
    ///         Err(err) => eprintln!("Error: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn list_models(&self) -> Result<ModelsResponse, reqwest::Error> {
        let models_url = self
            .api_url
            .join("/v1beta/models")
            .expect("Failed to construct models URL");

        let models_url_with_key = models_url
            .clone()
            .join(&format!("?key={}", self.api_key))
            .expect("Failed to construct models URL with key");

        let response = self
            .client
            .get(models_url_with_key.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await?;

        let json: ModelsResponse = response.json().await?;
        Ok(json)
    }
}
