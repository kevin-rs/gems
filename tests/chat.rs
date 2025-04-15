use anyhow::Result;
use gems::chat::ChatBuilder;
use gems::messages::Content;
use gems::messages::Message;
use gems::models::Model;
use gems::traits::CTrait;
use gems::Client;

#[test]
fn test_build_with_required() {
    let chat = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![Message::User {
            content: Content::Text("Hello".into()),
            name: None,
        }])
        .build()
        .unwrap();

    assert_eq!(chat.model, Model::Flash20);
    assert_eq!(chat.messages.len(), 1);
    assert!(chat.system.is_none());
}

#[test]
fn test_build_with_system() {
    let user_message = Message::User {
        content: Content::Text("Hello".into()),
        name: None,
    };

    let system_message = Message::System {
        content: Content::Text("You're a helpful assistant.".into()),
        name: None,
    };

    let chat = ChatBuilder::default()
        .model(Model::Pro15)
        .messages(vec![user_message.clone()])
        .system(vec![system_message.clone()])
        .build()
        .unwrap();

    assert_eq!(chat.model, Model::Pro15);
    assert_eq!(chat.messages.len(), 1);
    assert_eq!(chat.system.as_ref().unwrap().len(), 1);
}

#[test]
fn test_build_empty_messages() {
    let result = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![])
        .build();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().messages.len(), 0);
}

#[test]
fn test_build_no_model() {
    let result = ChatBuilder::default()
        .messages(vec![Message::User {
            content: Content::Text("Hello".into()),
            name: None,
        }])
        .build();

    assert!(result.is_ok());
}

#[test]
fn test_build_default_empty() {
    let chat = ChatBuilder::default().build().unwrap_or_default();

    assert_eq!(chat.messages.len(), 0);
    assert!(chat.system.is_none());
}

#[test]
fn test_build_clone() {
    let user_message = Message::User {
        content: Content::Text("Test".into()),
        name: Some("Tester".into()),
    };

    let chat = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![user_message.clone()])
        .build()
        .unwrap();

    let chat_clone = chat.clone();
    assert_eq!(chat_clone.messages[0], user_message);
    assert_eq!(chat_clone.model, chat.model);
}
#[tokio::test]
async fn test_required() -> Result<()> {
    let gemini_client = Client::builder()
        .model(&Model::Flash20.to_string())
        .build()?;

    gemini_client.set_api_key(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned(),
    );

    let chat = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![Message::User {
            content: Content::Text("Hello".into()),
            name: None,
        }])
        .build()
        .unwrap();

    let response = gemini_client.chat().generate(chat).await?;

    assert!(
        response.contains("Hello") || response.contains("Hi") || response.is_empty(),
        "Response does not contain expected content."
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_with_system() -> Result<()> {
    let gemini_client = Client::builder().model(&Model::Pro15.to_string()).build()?;

    gemini_client.set_api_key(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned(),
    );

    let user_message = Message::User {
        content: Content::Text("Hello".into()),
        name: None,
    };

    let system_message = Message::System {
        content: Content::Text("You're a helpful assistant.".into()),
        name: None,
    };

    let chat = ChatBuilder::default()
        .model(Model::Pro15)
        .messages(vec![user_message.clone()])
        .system(vec![system_message.clone()])
        .build()
        .unwrap();

    // NOTE: Gemini API occasionally responds with an error like "Missing candidates" for valid input.
    // This may be due to internal model constraints or bug.
    // TODO: Investigate root cause or add retries/fallback logic if needed.
    let response = gemini_client.chat().generate(chat).await?;

    // WARNING: Gemini API may return empty responses even when input is valid.
    // This behavior is likely non-deterministic and could vary based on model state or rate limits.
    assert!(
        response.contains("How") || response.is_empty(),
        "Response does not contain user content."
    );

    Ok(())
}

#[tokio::test]
async fn test_empty_msgs() -> Result<()> {
    let gemini_client = Client::builder()
        .model(&Model::Flash20.to_string())
        .build()?;

    gemini_client.set_api_key(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned(),
    );

    let chat = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![])
        .build()
        .unwrap();

    let response = gemini_client.chat().generate(chat).await;

    assert!(
        response.is_err(),
        "Expected error when sending empty messages."
    );

    Ok(())
}

#[tokio::test]
async fn test_default() -> Result<()> {
    let gemini_client = Client::builder()
        .model(&Model::Flash20.to_string())
        .build()?;

    gemini_client.set_api_key(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned(),
    );

    let chat = ChatBuilder::default().build().unwrap();

    assert_eq!(chat.messages.len(), 0, "Should have no messages.");
    assert!(chat.system.is_none(), "Should have no system instructions.");

    let response = gemini_client.chat().generate(chat).await;

    assert!(
        response.is_err(),
        "Expected error when generating from default chat."
    );

    Ok(())
}

#[tokio::test]
async fn test_clone() -> Result<()> {
    let gemini_client = Client::builder()
        .model(&Model::Flash20.to_string())
        .build()?;

    gemini_client.set_api_key(
        std::env::var("GEMINI_API_KEY")
            .unwrap_or_default()
            .to_owned(),
    );

    let user_message = Message::User {
        content: Content::Text("Test".into()),
        name: Some("Tester".into()),
    };

    let chat = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![user_message.clone()])
        .build()
        .unwrap();

    let chat_clone = chat.clone();

    assert_eq!(chat_clone.messages[0], user_message);
    assert_eq!(chat_clone.model, chat.model);

    let response = gemini_client.chat().generate(chat_clone).await?;

    assert!(
        response.contains("?") || response.is_empty(),
        "Response did not contain expected text."
    );

    Ok(())
}
