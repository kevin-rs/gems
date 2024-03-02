use serde::{Deserialize, Serialize};

/// Request structure for content generation.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    /// The model to be used for content generation.
    pub model: String,

    /// List of content items for generation.
    pub contents: Vec<Content>,
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
/// Structure representing a part of the content.
#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    /// The text of the content part.
    pub text: String,
}

/// Structure representing a candidate content.
#[derive(Debug, Deserialize)]
pub struct Candidate {
    /// The content of the candidate.
    pub content: Content,
}
