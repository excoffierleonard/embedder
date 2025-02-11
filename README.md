# [Embedder](https://embedder.excoffierleonard.com)

REST API service that takes in texts and returns their vector embeddings.

The goal of this project is to provide a central hub for embedding services.

Supports both local and OpenAI embedding models, batch processing, and fallbacks.

Demonstration Endpoint: [https://embedder.excoffierleonard.com/embed](https://embedder.excoffierleonard.com/embed)

```bash
curl https://embedder.excoffierleonard.com/embed \
     -H "Content-Type: application/json" \
     -d '{"texts": ["Hello, World!"]}'
```

## 📚 Table of Contents

- [Features](#-features)
- [Prerequisites](#-prerequisites)
- [Configuration](#-configuration)
- [Deployment](#-deployment)
- [API Documentation](#-api-documentation)
- [Development](#-development)
- [License](#-license)

## 📦 Features

- All Ollamas Embeddding Models (nomic-text-embed is pre-installed)
- All OpenAI Embedding Models (Necessite a valid API Key)

## 🛠 Prerequisites

For local build:

- [Rust](https://www.rust-lang.org/learn/get-started)
- Libraries:
  - OpenSSL development libraries

For deployment:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## ⚙ Configuration

The service can be configured using the following environment variables.

- `EMBEDDER_APP_PORT`: _INT_, The port on which the program listens on. (default: 8080)
- `OLLAMA_API_URL`: _STRING_, The Embedding Endpoint URL of the ollama instance. (default: `http://localhost:11434/api/embed` or `http://ollama:11434/api/embed` if using docker compose)
- `OPENAI_API_KEY`: _STRING_, The OpenAI API Key used as a fallback if none is provided in the request header.

> **⚠️ Warning:** Do not set a fallback OpenAI API key if you plan on exposing the service publicly.

## 🚀 Deployment

```bash
curl -o compose.yaml https://raw.githubusercontent.com/excoffierleonard/embedder/refs/heads/main/compose.yaml && \
docker compose up -d
```

## 📖 API Documentation

API documentation and examples are available in [docs/api.md](docs/api.md).

## 🧪 Development

Useful commands for development:

- Full build:

```bash
chmod +x ./scripts/build.sh && \
./scripts/build.sh
```

- Deployment tests:

```bash
chmod +x ./scripts/deploy-tests.sh && \
./scripts/deploy-tests.sh
```

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.