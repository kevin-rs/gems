#[macro_use]
extern crate rocket;
#[macro_use]
extern crate dotenv_codegen;
use dotenv;
use gems::Client;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![
        Method::Get,
        Method::Post,
        Method::Options,
        Method::Put,
        Method::Delete,
    ]
    .into_iter()
    .map(From::from)
    .collect();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        ..rocket_cors::CorsOptions::default()
    }
    .to_cors()
    .expect("Error in CORS setup");

    let api_key = dotenv!("GEMINI_API_KEY");
    let model = dotenv!("GEMINI_MODEL");

    let client = Client::new(api_key, model);
    let routes = all_routes();
    rocket::build()
        .mount("/", routes)
        .attach(cors)
        .manage(client)
        .launch()
        .await
        .expect("Launch Error");
    Ok(())
}

fn all_routes() -> Vec<rocket::Route> {
    routes![
        generate_content,
        stream_generate_content,
        count_tokens,
        embed_content
    ]
}

#[post(
    "/gems/generate-content",
    format = "application/json",
    data = "<input_text>"
)]
async fn generate_content(client: &State<Client>, input_text: String) -> String {
    let mut client = Client::new(&client.api_key, &client.model);
    match client.generate_content(&input_text).await {
        Ok(response) => response,
        Err(error) => error.to_string(),
    }
}

#[post(
    "/gems/stream-generate-content",
    format = "application/json",
    data = "<input_text>"
)]
async fn stream_generate_content(client: &State<Client>, input_text: String) -> String {
    let mut client = Client::new(&client.api_key, &client.model);

    client.stream_generate_content(&input_text, true).await.unwrap()
}

#[post(
    "/gems/count-tokens",
    format = "application/json",
    data = "<input_text>"
)]
async fn count_tokens(client: &State<Client>, input_text: String) -> String {
    let mut client = Client::new(&client.api_key, &client.model);
    match client.count_tokens(&input_text).await {
        Ok(response) => response.to_string(),
        Err(error) => error.to_string(),
    }
}

#[post(
    "/gems/embed-content",
    format = "application/json",
    data = "<input_text>"
)]
async fn embed_content(client: &State<Client>, input_text: String) -> Json<Vec<f64>> {
    let mut client = Client::new(&client.api_key, "embedding-001");
    let response = client.embed_content(&input_text).await.unwrap();
    Json(response.embedding.unwrap().values)
}
