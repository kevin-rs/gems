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
- [Examples](#-examples)
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

### Analyze an image and generate content from text:

```sh
curl -o image.jpg https://storage.googleapis.com/generativeai-downloads/images/scones.jpg

gems vision -i image.jpg

gems vision -i image.jpg -t "What's in the image?"
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
| `vision`                | Analyze an image and generate content from text.         |
| `stream`                | Stream the generation of content.                        |
| `count`                 | Count the number of tokens in a text.                    |
| `embed`                 | Embed content into a specified model.                    |
| `batch`                 | Batch embed multiple contents.                           |
| `info`                  | Get information about the current model.                 |
| `list`                  | List available models.                                   |


## ✨ Usage as Dependency


1. Add the `gems` crate:

   ```toml
   [dependencies]
   gems = "0.0.9"
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

## 📌 Examples

This repository contains a list of notebooks examples on how to use the sdk and or the cli. To use the notebooks in this repository, you need to set up your environment. Follow these steps to get started:

1. Clone the repository to your local machine:

   ```sh
   git clone https://github.com/wiseaidev/gems.git
   ```

1. Install the required dependencies and libraries. Make sure you have [`Rust`](https://rustup.rs/), [`Jupyter Notebook`](https://jupyter.org/install), and [`evcxr_jupyter`](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md) installed on your system.

   ```sh
   # Install a Rust toolchain (e.g. nightly):
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
 
   # Install Jupyter Notebook
   pip install notebook
 
   # Install evcxr_jupyter
   cargo install evcxr_jupyter
   evcxr_jupyter --install 
   ```

1. Navigate to the cloned repository and build the project:

   ```sh
   cd gems
   cargo build --release --all-features
   ```

1. Start Jupyter Notebook:

   ```sh
   jupyter notebook
   ```

1. Access the notebooks in your web browser by clicking on the notebook file you want to explore.

| ID | Example | Open on GitHub | Launch on Binder | Launch on Colab |
|----|---------------|-----------|:-------------|-------------|
| 1  | **Basic** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/basic.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/wiseaidev/gems/main?filepath=examples/basic.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/wiseaidev/gems/blob/main/examples/basic.ipynb) |
| 2  | **Rocket** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/rocket.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/wiseaidev/gems/main?filepath=examples/rocket.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/wiseaidev/gems/blob/main/examples/rocket.ipynb) |
| 3  | **Axum** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/axum.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/wiseaidev/gems/main?filepath=examples/axum.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/wiseaidev/gems/blob/main/examples/axum.ipynb) |

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/gems). Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
