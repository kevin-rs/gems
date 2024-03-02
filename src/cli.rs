//! This module contains the CLI functionalities for interacting with the Gemini API.

#[cfg(feature = "cli")]
use clap::builder::styling::{AnsiColor, Effects, Styles};
#[cfg(feature = "cli")]
use clap::{Args, Parser, Subcommand};

#[cfg(feature = "cli")]
fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[cfg(feature = "cli")]
#[derive(Parser, Debug, Clone)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "gems",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about=r#"
💎 GEMS
=======

A command-line tool for interacting with the Google Gemini API.

FUNCTIONALITIES:
  - Generate Content: Generate content using the Gemini API.
  - Stream Generate Content: Stream content generation using the Gemini API.
  - Count Tokens: Count tokens in a given text using the Gemini API.
  - Embed Content: Embed content using the Gemini API.
  - Batch Embed Contents: Batch embed contents using the Gemini API.
  - Get Model Info: Get information about the model from the Gemini API.
  - List Models: List available models from the Gemini API.

USAGE:
  gems [OPTIONS] <COMMAND>

EXAMPLES:
  Generate content:
    gems generate -t "Hello"

  Stream generate content:
    gems stream -t "Generate a short fictional story"

  Count tokens:
    gems count -t "Hello There!"

  Embed content:
    gems -m 'embedding-001' embed -t "Write a story about a magic backpack."

  Batch embed contents:
    gems -m 'embedding-001' batch -t "Write a story about a magic backpack.","Generate a poem about nature."

  Get model info:
    gems info

  List models:
    gems list

For more information, visit: github.com/wiseaidev/gems
"#
)]
pub struct Cli {
    /// API key for authentication.
    #[arg(short, long)]
    pub api_key: Option<String>,
    /// Model to be used.
    #[arg(short, long)]
    pub model: Option<String>,
    #[command(subcommand)]
    pub cmd: Command,
}

#[cfg(feature = "cli")]
#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Generate(Generate),
    Stream(Stream),
    Count(Count),
    Embed(Embed),
    Batch(Batch),
    Info(Info),
    List(List),
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Generate {
    /// The text to generate content from.
    #[arg(short, long)]
    pub text: String,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Stream {
    /// The text to generate content from.
    #[arg(short, long)]
    pub text: String,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Count {
    /// The text to count tokens from.
    #[arg(short, long)]
    pub text: String,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Embed {
    /// The text to embed content from.
    #[arg(short, long)]
    pub text: String,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Batch {
    /// List of texts to embed contents from.
    #[arg(short, long)]
    pub texts: Vec<String>,
}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct Info {}

#[cfg(feature = "cli")]
#[derive(Args, Debug, Clone)]
pub struct List {}
