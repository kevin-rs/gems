use anyhow::Result;
use gems::embed::{BatchEmbeddingBuilder, EmbeddingBuilder};
use gems::messages::{Content, Message};
use gems::models::Model;
use gems::traits::CTrait;
use gems::Client;

#[test]
fn test_embed_build_required() {
    let message = Message::User {
        content: Content::Text("Test input".into()),
        name: None,
    };

    let embed = EmbeddingBuilder::default()
        .model(Model::Embedding)
        .input(message.clone())
        .build()
        .unwrap();

    assert_eq!(embed.model, Model::Embedding);
    assert_eq!(embed.input, message);
}

#[test]
fn test_batch_embed_build() {
    let inputs = vec![
        Message::User {
            content: Content::Text("First".into()),
            name: None,
        },
        Message::User {
            content: Content::Text("Second".into()),
            name: None,
        },
    ];

    let batch = BatchEmbeddingBuilder::default()
        .model(Model::Embedding)
        .input(inputs.clone())
        .build()
        .unwrap();

    assert_eq!(batch.model, Model::Embedding);
    assert_eq!(batch.input, inputs);
}

#[tokio::test]
async fn test_embed_create() -> Result<()> {
    let gemini_client = Client::builder()
        .model(&Model::Embedding.to_string())
        .build()?;

    gemini_client.set_api_key(std::env::var("GEMINI_API_KEY").unwrap_or_default());

    let input = Message::User {
        content: Content::Text("What is Rust programming?".into()),
        name: None,
    };

    let params = EmbeddingBuilder::default()
        .model(Model::Embedding)
        .input(input)
        .build()?;

    let response = gemini_client.embeddings().create(params).await?;

    let embeddings = response.embedding.clone().unwrap_or_default().values;

    assert!(
        !embeddings.is_empty(),
        "Expected embedding vector in response."
    );

    for &value in &embeddings {
        assert!(
            value.is_finite(),
            "Embedding value should be finite (not NaN or infinite)"
        );
        assert!(
            value >= -1.0 && value <= 1.0,
            "Embedding value out of expected range: {}",
            value
        );
    }

    assert_eq!(embeddings.len(), 768, "Unexpected embedding vector size");

    Ok(())
}

#[tokio::test]
async fn test_embed_batch() -> Result<()> {
    let gemini_client = Client::builder()
        .model(&Model::Embedding.to_string())
        .build()?;

    gemini_client.set_api_key(std::env::var("GEMINI_API_KEY").unwrap_or_default());

    let inputs = vec![
        Message::User {
            content: Content::Text("First embedding".into()),
            name: None,
        },
        Message::User {
            content: Content::Text("Second embedding".into()),
            name: None,
        },
    ];

    let params = BatchEmbeddingBuilder::default()
        .model(Model::Embedding)
        .input(inputs)
        .build()?;

    let response = gemini_client.embeddings().batch(params).await?;

    assert_eq!(
        response.embeddings.unwrap_or_default().len(),
        2,
        "Expected two embeddings in batch response."
    );

    Ok(())
}
