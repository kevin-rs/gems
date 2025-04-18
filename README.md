<div align="center">

# 💎 Gems

[![Crates.io](https://img.shields.io/crates/v/gems.svg)](https://crates.io/crates/gems)
[![docs](https://docs.rs/gems/badge.svg)](https://docs.rs/gems/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

| 🐧 Linux `(Recommended)` | 🪟 Windows |
| :------: | :--------: |
| ![gems-demo](https://github.com/user-attachments/assets/c446c29f-d4c8-4ee0-9e3d-951310e2b972) | ![gems-demo](https://github.com/user-attachments/assets/e942d1ad-7df6-4532-b22f-d4c586e64c8a) |
| [Download Executable File](https://github.com/kevin-rs/gems/releases/download/v0.1.3/gems) | [Download `.exe` File](https://github.com/kevin-rs/gems/releases/download/v0.1.3/gems.exe) |
| `cargo install gems --all-features` | `cargo install gems --all-features` |

</div>

<video src="https://github.com/user-attachments/assets/cd17c52a-086c-4d21-9129-93f6ec2df61d"></video>

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
- Generate images.
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
gems embed -t "Write a story about a magic backpack."
```

### Batch embed multiple contents:

```sh
gems batch -t "Write a story about a magic backpack.","Generate a poem about nature."
```

### Get information about the current model:

```sh
gems info
```

### List available models:

```sh
gems list
```

### Generate an Image

```sh
gems imagen -t "Hi, can you create a 3d rendered image of a pig with wings and a top hat flying over a happy futuristic scifi city with lots of greenery?"
```

### TUI mode

```sh
gems
```

## 🎨 Options

| Option                   | Description                                              |
|--------------------------|----------------------------------------------------------|
| ``                       | TUI mode.                                                |
| `--api-key`              | Specify the API key for accessing the Gemini API.        |
| `--model`                | Specify the model to use for generating content.         |


## 🛠 Subcommands

| Subcommand              | Description                                              |
|-------------------------|----------------------------------------------------------|
| `generate`              | Generate creative content.                               |
| `vision`                | Analyze an image and generate content from text.         |
| `stream`                | Stream the generation of content.                        |
| `imagen`                | Generate an image.                                       |
| `count`                 | Count the number of tokens in a text.                    |
| `embed`                 | Embed content into a specified model.                    |
| `batch`                 | Batch embed multiple contents.                           |
| `info`                  | Get information about the current model.                 |
| `list`                  | List available models.                                   |


## ✨ Usage as Dependency


1. Add the `gems` crate:

   ```toml
   [dependencies]
   gems = "0.1.3"
   ```

1. Use the `Client` struct to interact with the Gemini API:

   ```rust
   use gems::Client;
   use gems::traits::CTrait;
   use gems::messages::Content;
   use gems::messages::Message;
   use gems::models::Model;
   use gems::chat::ChatBuilder;
   use anyhow::Result;

   #[tokio::main]
   async fn main() -> Result<()> {
        let mut gemini_client = Client::builder().model("your-model").build()?;

        gemini_client.set_api_key("your-api-key".to_string());

      let parameters = ChatBuilder::default()
          .model(Model::Flash20)
          .messages(vec![Message::User {
              content: Content::Text("Hello".to_string()),
              name: None,
          }])
          .build()?;

       match gemini_client.chat().generate(parameters).await {
           Ok(response) => {
               println!("{}", response);
           }
           Err(err) => {
               eprintln!("Error: {:?}", err);
           }
       }

       Ok(())
   }
   ```

## 📌 Examples

This repository contains a list of notebooks examples on how to use the sdk and or the cli. To use the notebooks in this repository, you need to set up your environment. Follow these steps to get started:

1. Clone the repository to your local machine:

   ```sh
   git clone https://github.com/kevin-rs/gems.git
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
| 1  | **Basic** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/basic.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/kevin-rs/gems/main?filepath=examples/basic.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/kevin-rs/gems/blob/main/examples/basic.ipynb) |
| 2  | **Rocket** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/rocket.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/kevin-rs/gems/main?filepath=examples/rocket.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/kevin-rs/gems/blob/main/examples/rocket.ipynb) |
| 3  | **Axum** | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./examples/axum.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/kevin-rs/gems/main?filepath=examples/axum.ipynb) |  [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/kevin-rs/gems/blob/main/examples/axum.ipynb) |

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/kevin-rs/gems). Your contributions help improve this crate for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
