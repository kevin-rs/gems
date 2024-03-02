# 💎 Gems

[![Crates.io](https://img.shields.io/crates/v/gems.svg)](https://crates.io/crates/gems)
[![docs](https://docs.rs/gems/badge.svg)](https://docs.rs/gems/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> 💎 GEMS: A cli, tui, and sdk for interacting with the Gemini API, allowing you to generate creative content, perform text-related tasks, and get information about supported models.

## 📖 Table of Contents

- [Installation](#-installation)
- [Features](#-features)
- [Usage](#-usage-as-cli)
- [Options](#-options)
- [Subcommands](#-subcommands)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install the `gems` cli, use the following Cargo command:

```bash
cargo install --locked gems --all-features
```

## ✨ Features

- Interact with the Gemini API from the terminal.
- Generate creative content with ease.
- Stream generation of content for continuous output.
- Count the number of tokens in a text.
- Embed content into a specified model.
- Batch embed multiple contents efficiently.
- Get information about the current model and list available models.

## Usage

Before using the `gems` CLI, make sure to set the following environment variables:

```bash
export GEMINI_API_KEY=<your_gemini_api_key>
export GEMINI_MODEL=<your_gemini_model>
```

Generate an api key from [Google AI Studio](https://aistudio.google.com/app/apikey).

## ⌨ Usage as CLI

### Generate creative content:

```sh
gems generate -t "Hello"
```

### Stream generation of content:

```sh
gems stream -t "Generate a short fictional story"
```

### Count the number of tokens in a text:

```sh
gems count -t "Hello There!"
```

### Embed content into a specified model:

```sh
gems -m 'embedding-001' embed -t "Write a story about a magic backpack."
```

### Batch embed multiple contents:

```sh
gems -m 'embedding-001' batch -t "Write a story about a magic backpack.","Generate a poem about nature."
```

### Get information about the current model:

```sh
gems info
```

### List available models:

```sh
gems list
```

## 🎨 Options

| Option                   | Description                                              |
|--------------------------|----------------------------------------------------------|
| `--api-key`              | Specify the API key for accessing the Gemini API.        |
| `--model`                | Specify the model to use for generating content.         |


## 🛠 Subcommands

| Subcommand              | Description                                              |
|-------------------------|----------------------------------------------------------|
| `generate`              | Generate creative content.                               |
| `stream`                | Stream the generation of content.                        |
| `count`                 | Count the number of tokens in a text.                    |
| `embed`                 | Embed content into a specified model.                    |
| `batch`                 | Batch embed multiple contents.                           |
| `info`                  | Get information about the current model.                 |
| `list`                  | List available models.                                   |

### ✨ Usage as Dependency

1. Add the `gems` crate:

   ```toml
   [dependencies]
   gems = "0.0.1"
   ```

1. Use the `Client` struct to interact with the Gemini API:

   ```rust
   use gems::Client;
 
   #[tokio::main]
   async fn main() {
       let mut client = Client::new("your_api_key", "your_model");
 
       // Use the various functions provided by the client
       // For example:
       match client.generate_content("Hello").await {
           Ok(response) => {
               println!("{}", response);
           }
           Err(err) => {
               eprintln!("Error: {:?}", err);
           }
       }
   }
   ```

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/gems). Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
