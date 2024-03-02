# Gems Rust SDK Rocket Demo

🚀This project is a demo of using the `gems` SDK with the Rocket web framework.

## Getting Started

### Prerequisites

🔧 Before getting started, make sure you have the following prerequisites:

- Rust installed on your system.

### Installation

This project includes a `Makefile` to simplify the installation process. Ensure that you have the `make` utility available on your system.

1. Clone the project repository:

   ```shell
   git clone https://github.com/wiseaidev/gems.git
   cd gems/examples/rocket
   ```

1. Install the crate and all required core dependencies:

   ```shell
   make install
   ```

   If the `.env` file doesn't exist, this command creates it by copying from `.env.example`. This command also builds the project. Now set the following environment variables:

   ```env
   GEMINI_API_KEY=your_gemini_api_key
   GEMINI_MODEL=your_gemini_model
   ```

1. Run the server locally:

   ```shell
   make run
   ```

## Usage

🚀 To use this project, follow the steps below for installation and then run the server locally.

## API Endpoints

This project provides the following API endpoints:

### Generate Content

This endpoint generates content based on input text.

- **HTTP Method**: POST
- **Path**: `/gems/generate-content`
- **Request Format**: JSON
- **Example**:

  ```shell
  curl -X POST -H "Content-Type: application/json" -d '{"input_text": "Hello There!"}' http://127.0.0.1:8000/gems/generate-content
  ```

### Stream Generate Content

This endpoint asynchronously generates content based on input text.

- **HTTP Method**: POST
- **Path**: `/gems/stream-generate-content`
- **Request Format**: JSON
- **Example**:

  ```shell
  curl -X POST -H "Content-Type: application/json" -d '{"input_text": "Hello There!"}' http://127.0.0.1:8000/gems/stream-generate-content
  ```

### Count Tokens

This endpoint counts tokens in the input text.

- **HTTP Method**: POST
- **Path**: `/gems/count-tokens`
- **Request Format**: JSON
- **Example**:

  ```shell
  curl -X POST -H "Content-Type: application/json" -d '{"input_text": "Hello There!"}' http://127.0.0.1:8000/gems/count-tokens
  ```

### Embed Content

This endpoint embeds content based on the input text.

- **HTTP Method**: POST
- **Path**: `/gems/embed-content`
- **Request Format**: JSON
- **Example**:

  ```shell
  curl -X POST -H "Content-Type: application/json" -d '{"input_text": "Hello There!}' http://127.0.0.1:8000/gems/embed-content
  ```

## License

📜 This project is licensed under the [MIT](LICENSE) license - see the [LICENSE](LICENSE) file for details.