use crate::chat::Chats;
use crate::embed::Embeddings;
use crate::imagen::Images;
use crate::models::Model;
use crate::models::Models;
use crate::stream::Streaming;
use crate::tokens::Tokens;
use crate::vision::Visions;
use anyhow::Result;
use reqwest::{Method, RequestBuilder};

pub trait CTrait {
    fn set_api_key(&self, api_key: String);
    fn get_api_key(&self) -> Option<String>;
    fn get_model(&self) -> Model;

    fn set_model(&mut self, model: Model);
    fn request(&self, method: Method, endpoint: &str) -> Result<RequestBuilder>;
    fn chat(&self) -> Chats;
    fn embeddings(&self) -> Embeddings;
    fn tokens(&self) -> Tokens;
    fn vision(&self) -> Visions;
    fn stream(&self) -> Streaming;
    fn models(&self) -> Models;
    fn images(&self) -> Images;
}
