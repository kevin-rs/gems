use axum::{
    extract::Extension,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::method_routing::post,
    Json, Router,
};
use dotenv_codegen::dotenv;
use gems::Client;
use http::header::CONTENT_TYPE;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let allowed_origins = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    let api_key = dotenv!("GEMINI_API_KEY");
    let model = dotenv!("GEMINI_MODEL");

    let client = Client::new(api_key, model);
    let routes = all_routes();

    let trace = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(
        listener,
        routes
            .layer(allowed_origins)
            .layer(Extension(client))
            .layer(trace),
    )
    .await
    .expect("Server failed to start");
}

fn all_routes() -> Router {
    Router::new()
        .route("/gems/generate-content", post(generate_content))
        .route(
            "/gems/stream-generate-content",
            post(stream_generate_content),
        )
        .route("/gems/count-tokens", post(count_tokens))
        .route("/gems/embed-content", post(embed_content))
        .route("/", post(|| async { StatusCode::NOT_FOUND }))
}

#[derive(Debug, serde::Deserialize)]
struct GenerateContentRequest {
    input_text: String,
}

async fn generate_content(
    Extension(mut client): Extension<Client>,
    Json(request): Json<GenerateContentRequest>,
) -> impl IntoResponse {
    match client.generate_content(&request.input_text).await {
        Ok(response) => response,
        Err(error) => error.to_string(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct StreamGenerateContentRequest {
    input_text: String,
}

async fn stream_generate_content(
    Extension(mut client): Extension<Client>,
    Json(request): Json<StreamGenerateContentRequest>,
) -> impl IntoResponse {
    match client
        .stream_generate_content(&request.input_text, true)
        .await
    {
        Ok(response) => response,
        Err(error) => error.to_string(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct CountTokensRequest {
    input_text: String,
}

async fn count_tokens(
    Extension(mut client): Extension<Client>,
    Json(request): Json<CountTokensRequest>,
) -> impl IntoResponse {
    match client.count_tokens(&request.input_text).await {
        Ok(response) => response.to_string(),
        Err(error) => error.to_string(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct EmbedContentRequest {
    input_text: String,
}

async fn embed_content(
    Extension(client): Extension<Client>,
    Json(request): Json<EmbedContentRequest>,
) -> impl IntoResponse {
    let mut client = Client::new(&client.api_key, "embedding-001");
    let response = client.embed_content(&request.input_text).await.unwrap();
    Json(response.embedding.unwrap().values)
}
