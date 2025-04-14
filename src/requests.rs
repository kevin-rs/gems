use serde::{Deserialize, Serialize};

/// Request structure for content generation.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    /// The model to be used for content generation.
    pub model: String,

    /// List of content items for generation.
    pub contents: Vec<Content>,

    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    pub config: Option<GenerationConfig>,
}

/// Request structure for content embedding.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiEmbedRequest {
    /// The model to be used for content embedding.
    pub model: String,

    /// Content item for embedding.
    pub content: Content,
}
/// Request structure for batch content embedding.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiEmbedRequests {
    /// List of requests for batch content embedding.
    pub requests: Vec<GeminiEmbedRequest>,
}
/// Structure representing content information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    /// List of parts composing the content.
    pub parts: Vec<Part>,
}

/// Define an enum to represent different types of parts in the content.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Part {
    /// Represents a text part in the content.
    Text { text: String },
    /// Represents an image part in the content.
    Image { inline_data: Option<ImageContent> },
}

/// Implement the `Part` enum for the `Serialize` trait
impl Part {
    /// Create a new `Part` with text content.
    pub fn text(text: &str) -> Self {
        Part::Text {
            text: text.to_string(),
        }
    }

    /// Create a new `Part` with image content.
    pub fn image(inline_data: Option<ImageContent>) -> Self {
        Part::Image { inline_data }
    }
}

/// Structure representing a candidate content.
#[derive(Debug, Deserialize)]
pub struct Candidate {
    /// The content of the candidate.
    pub content: Content,
}

/// Structure representing the image part of the Gemini request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageContent {
    /// The MIME type of the image.
    pub mime_type: String,
    /// The actual image data in a base64-encoded string.
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationConfig {
    #[serde(rename = "responseModalities")]
    pub response_modalities: Vec<String>,
}
