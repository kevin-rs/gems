use anyhow::Result;
use gems::imagen::ImageGenBuilder;
use gems::messages::{Content, Message};
use gems::models::Model;
use gems::traits::CTrait;
use gems::Client;

#[test]
fn test_imagegen_build_required() {
    let user_msg = Message::User {
        content: Content::Text("A cat sitting on a beach".into()),
        name: None,
    };

    let imagegen = ImageGenBuilder::default()
        .model(Model::FlashExpImage)
        .input(user_msg.clone())
        .build()
        .unwrap();

    assert_eq!(imagegen.model, Model::FlashExpImage);
    assert_eq!(imagegen.input, user_msg);
    assert!(imagegen.system.is_none());
}

#[test]
fn test_imagegen_build_with_system() {
    let user_msg = Message::User {
        content: Content::Text("A futuristic cityscape".into()),
        name: None,
    };

    let system_msg = Message::System {
        content: Content::Text("Only respond with image content.".into()),
        name: None,
    };

    let imagegen = ImageGenBuilder::default()
        .model(Model::FlashExpImage)
        .input(user_msg.clone())
        .system(vec![system_msg.clone()])
        .build()
        .unwrap();

    assert_eq!(imagegen.model, Model::FlashExpImage);
    assert_eq!(imagegen.input, user_msg);
    assert_eq!(imagegen.system.unwrap().len(), 1);
}

#[tokio::test]
async fn test_imagegen_generate_basic() -> Result<()> {
    let mut gemini_client = Client::builder().build()?;

    gemini_client.set_model(Model::FlashExpImage);
    gemini_client.set_api_key(std::env::var("GEMINI_API_KEY").unwrap_or_default());

    let input = Message::User {
        content: Content::Text("A sunset over a mountain range".into()),
        name: None,
    };

    let params = ImageGenBuilder::default()
        .model(Model::FlashExpImage)
        .input(input)
        .build()?;

    let image_bytes = gemini_client.images().generate(params).await?;

    assert!(!image_bytes.is_empty(), "Expected image data in response.");

    Ok(())
}

// TODO: Research why Gemini doesn't allow system prompts in image gen
#[tokio::test]
#[ignore]
async fn test_imagegen_generate_with_system() -> Result<()> {
    let mut gemini_client = Client::builder().build()?;

    gemini_client.set_model(Model::FlashExpImage);
    gemini_client.set_api_key(std::env::var("GEMINI_API_KEY").unwrap_or_default());

    let input = Message::User {
        content: Content::Text("A robot reading a book in a library".into()),
        name: None,
    };

    let system = Message::System {
        content: Content::Text("Use high-detail, photorealistic rendering.".into()),
        name: None,
    };

    let params = ImageGenBuilder::default()
        .model(Model::FlashExpImage)
        .input(input)
        .system(vec![system])
        .build()?;

    let image_bytes = gemini_client.images().generate(params).await?;

    assert!(
        !image_bytes.is_empty(),
        "Expected image data from system-influenced generation."
    );

    Ok(())
}
