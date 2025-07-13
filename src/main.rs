use anyhow::Result;

/// The main entry point of `gems`.
///
/// It parses command-line arguments using the `clap` crate, configures the client based on
/// the provided command-line options, and performs an operation using the specified subcommand.
#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "cli")]
    {
        use clap::Parser;
        use futures_util::StreamExt;
        use gems::chat::ChatBuilder;
        use gems::cli::{Cli, Command};
        use gems::embed::BatchEmbeddingBuilder;
        use gems::embed::EmbeddingBuilder;
        use gems::imagen::ImageGenBuilder;
        use gems::messages::Content;
        use gems::messages::Message;
        use gems::models::ModBuilder;
        use gems::models::Model;
        use gems::stream::StreamBuilder;
        use gems::tokens::TokenBuilder;
        use gems::traits::CTrait;
        use gems::tts::TtsGenBuilder;
        use gems::vidgen::VideoGenBuilder;

        use gems::tui::run_tui;
        use gems::utils::{
            extract_text_from_partial_json, load_and_encode_image, type_with_cursor_effect,
        };
        use gems::vision::VisionBuilder;
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
                .unwrap_or("gemini-2.0-flash".to_string())
                .to_owned()
        } else {
            args.model.unwrap().to_owned()
        };
        let mut gemini_client = Client::builder().model(&model).build()?;

        gemini_client.set_api_key(api_key);
        match args.cmd {
            Some(Command::Generate(cmd)) => {
                let parameters = ChatBuilder::default()
                    .model(Model::Flash20)
                    .messages(vec![Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    }])
                    .build()?;

                let response = gemini_client.chat().generate(parameters).await?;
                println!("{}", response);
            }
            Some(Command::Stream(cmd)) => {
                let parameters = StreamBuilder::default()
                    .model(Model::Flash20)
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .build()?;

                let response = gemini_client.stream().generate(parameters).await?;
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
                                if !line.is_empty() {
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
            }
            Some(Command::Count(cmd)) => {
                let params = TokenBuilder::default()
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .build()?;

                let count = gemini_client.tokens().count(params).await?;
                println!("Token Count: {:?}", count);
            }
            Some(Command::Embed(cmd)) => {
                let params = EmbeddingBuilder::default()
                    .model(Model::Embedding)
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .build()?;
                gemini_client.set_model(Model::Embedding);
                let response = gemini_client.embeddings().create(params).await?;
                println!("Embed Content: {:?}", response);
            }
            Some(Command::Batch(cmd)) => {
                let texts: Vec<Message> = cmd
                    .texts
                    .iter()
                    .map(|text| Message::User {
                        content: Content::Text(text.into()),
                        name: None,
                    })
                    .collect();
                let params = BatchEmbeddingBuilder::default()
                    .model(Model::Embedding)
                    .input(texts)
                    .build()?;

                gemini_client.set_model(Model::Embedding);
                let response = gemini_client.embeddings().batch(params).await?;
                println!("Batch Embed Contents: {:?}", response);
            }
            Some(Command::Vision(cmd)) => {
                let base64_image_data = match load_and_encode_image(&cmd.image) {
                    Ok(data) => data,
                    Err(_) => {
                        eprintln!("Error loading image!");
                        "".to_string()
                    }
                };
                let params = VisionBuilder::default()
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .image(Message::Tool {
                        content: base64_image_data,
                    })
                    .build()?;

                let result = gemini_client.vision().generate(params).await?;
                println!("{}", result);
            }
            Some(Command::Info(_)) => {
                let params = ModBuilder::default().model(Model::default()).build()?;
                let model_info = gemini_client.models().get(params).await?;
                model_info.print();
            }
            Some(Command::List(_)) => {
                let models = gemini_client.models().list().await?;
                models.print();
            }
            Some(Command::Imagen(cmd)) => {
                gemini_client.set_model(Model::FlashExpImage);

                let params = ImageGenBuilder::default()
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .model(Model::FlashExpImage)
                    .build()
                    .unwrap();

                let image_data = gemini_client.images().generate(params).await?;

                tokio::fs::write("output.png", &image_data).await?;
            }
            Some(Command::Vidgen(cmd)) => {
                gemini_client.set_model(Model::Veo2);

                let params = VideoGenBuilder::default()
                    .model(Model::Veo2)
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .build()
                    .unwrap();

                let bytes = gemini_client.videos().generate(params).await?;

                tokio::fs::write("output.mp4", &bytes).await?;
            }
            Some(Command::Tts(cmd)) => {
                gemini_client.set_model(Model::Tts);

                let params = TtsGenBuilder::default()
                    .model(Model::Tts)
                    .input(Message::User {
                        content: Content::Text(cmd.text),
                        name: None,
                    })
                    .voice(cmd.voice)
                    .build()
                    .unwrap();

                let bytes = gemini_client.tts().generate(params).await?;

                tokio::fs::write("output.pcm", &bytes).await?;
            }
            None => {
                let _ = run_tui().await;
            }
        }
    }
    Ok(())
}
