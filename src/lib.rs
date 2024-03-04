//! # Gems
//!
//! A crate for interacting with the Gemini API, allowing you to generate creative content, perform text-related tasks,
//! and get information about supported models.
//!
//! ## Quick Start
//!
//! Get started with the `gems` library by following these simple steps:
//!
//! 1. Install the `gems` crate by adding the following line to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! gems = "0.0.7"
//! ```
//!
//! 2. Use the `Client` struct to interact with the Gemini API:
//!
//! ```rust
//! use gems::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = Client::new("your_api_key", "your_model");
//!
//!     // Use the various functions provided by the client
//!     // For example:
//!     match client.generate_content("Hello").await {
//!         Ok(response) => {
//!             println!("{}", response);
//!         }
//!         Err(err) => {
//!             eprintln!("Error: {:?}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ## Subcommands
//!
//! | Subcommand              | Description                                              |
//! |-------------------------|----------------------------------------------------------|
//! | `generate`              | Generate creative content.                               |
//! | `vision`                | Analyze an image and generate content from text.         |
//! | `stream`                | Stream the generation of content.                        |
//! | `count`                 | Count the number of tokens in a text.                    |
//! | `embed`                 | Embed content into a specified model.                    |
//! | `batch`                 | Batch embed multiple contents.                            |
//! | `info`                  | Get information about the current model.                  |
//! | `list`                  | List available models.                                   |
//!
//! ## GitHub Repository
//!
//! You can access the source code for the `gems` crate on [GitHub](https://github.com/wiseaidev/gems).
//!
//! ## Contributing
//!
//! Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement,
//! please engage with the project on [GitHub](https://github.com/wiseaidev/gems).
//! Your contributions help improve this crate for the community.

pub mod client;
pub mod requests;
pub mod responses;
pub mod utils;

#[cfg(feature = "cli")]
pub mod cli;

pub use client::Client;
