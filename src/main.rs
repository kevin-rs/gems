/// The main entry point of `gems`.
///
/// It parses command-line arguments using the `clap` crate, configures the client based on
/// the provided command-line options, and performs an operation using the specified subcommand.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "cli")]
    {
        use clap::Parser;
        use gems::cli::{Cli, Command};
        use gems::Client;
        use std::env;
        let args: Cli = Cli::parse();

        let api_key = if args.api_key.is_none() {
            env::var("GEMINI_API_KEY").unwrap_or_default().to_owned()
        } else {
            args.api_key.unwrap().to_owned()
        };

        let model = if args.model.is_none() {
            env::var("GEMINI_MODEL")
                .unwrap_or("gemini-pro".to_string())
                .to_owned()
        } else {
            args.model.unwrap().to_owned()
        };
        let mut gemini_client = Client::new(&api_key, &model);

        match args.cmd {
            Command::Generate(cmd) => {
                let response = gemini_client.generate_content(&cmd.text).await?;
                println!("{}", response);
            }
            Command::Stream(cmd) => {
                gemini_client.stream_generate_content(&cmd.text).await?;
            }
            Command::Count(cmd) => {
                let count = gemini_client.count_tokens(&cmd.text).await?;
                println!("Token Count: {:?}", count);
            }
            Command::Embed(cmd) => {
                let count = gemini_client.embed_content(&cmd.text).await?;
                println!("Embed Content: {:?}", count);
            }
            Command::Batch(cmd) => {
                let texts: Vec<&str> = cmd.texts.iter().map(|text| text.as_str()).collect();
                let count = gemini_client.batch_embed_contents(texts).await?;
                println!("Batch Embed Contents: {:?}", count);
            }
            Command::Info(_) => {
                let model_info = gemini_client.get_model_info().await?;
                model_info.print();
            }
            Command::List(_) => {
                let models = gemini_client.list_models().await?;
                models.print();
            }
        }
    }
    Ok(())
}
