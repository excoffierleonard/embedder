# Embedder API Documentation

## Endpoints

1. [Embed the given texts](#embed-the-given-texts)

## Embed the given texts

### Request

#### Endpoint

```http
POST /embed
```

#### Header

> **Note:** The authorization header may be optional based on the wanted model and system configuration.

```
Authorization: Bearer *OPENAI_API_KEY*
```

#### Body (application/json)

```json
{
    "model": "*Optional Name of the embedding model*",
    "texts": [
        "*First Text.*",
        "*Seconf Text.*"
    ]
}
```

### Response

#### Status Codes

| Code | Name |  Description |
|-|-|-|
| `200` | `OK` | Successfully embedded the texts |
| `400` | `Bad Request` | Invalid request |
| `500` | `Internal Server Error` | Embedding failed |

#### Body

##### Success

```json
{
    "model": "*Name of the embedding model*",
    "embeddings": [
        [*Float1*, *Float2*, *Float3*],
        [*Float1*, *Float2*, *Float3*]
    ]
}
```

##### Error

```json
{
    "message": "*Description of the error.*"
}
```

### Examples

> **Note:** The embedding vectors were truncated due to being too long.

> **Note:** The api key was truncated due to being too long.

#### No model defined (Fallback to default Ollama model)

##### Request

```bash
curl --request POST \
     --url "http://localhost:8080/embed" \
     --header "Content-Type: application/json" \
     --data '{
        "texts": [
            "Hell World!",
            "Goodbye, World!"
        ]
     }'
```

##### Response

```json
{
    "model": "nomic-embed-text",
    "embeddings": [
        [0.0222439254, -0.015302311, 0.001980035],
        [-0.015302311, 0.0233868278, 0.045129864]
    ]
}
```

#### Using Ollama models

##### Request

```bash
curl --request POST \
     --url "http://localhost:8080/embed" \
     --header "Content-Type: application/json" \
     --data '{
        "model": "nomic-embed-text",
        "texts": [
            "Hell World!",
            "Goodbye, World!"
        ]
     }'
```

##### Response

```json
{
    "model": "nomic-embed-text",
    "embeddings": [
        [0.0222439254, -0.015302311, 0.001980035],
        [-0.015302311, 0.0233868278, 0.045129864]
    ]
}
```

#### Using OpenAI models with custom API Key

##### Request

```bash
curl --request POST \
     --url "http://localhost:8080/embed" \
     --header "Content-Type: application/json" \
     --header "Authorization: Bearer sk-proj-Agtp8k8" \
     --data '{
        "model": "text-embedding-3-large",
        "texts": [
            "Hell World!",
            "Goodbye, World!"
        ]
     }'
```

##### Response

```json
{
    "model": "text-embedding-3-large",
    "embeddings": [
        [0.0222439254, -0.015302311, 0.001980035],
        [-0.015302311, 0.0233868278, 0.045129864]
    ]
}
```

#### Using OpenAI models with no API Key defined, defaults to the system configuration if there is one

##### Request

```bash
curl --request POST \
     --url "http://localhost:8080/embed" \
     --header "Content-Type: application/json" \
     --data '{
        "model": "text-embedding-3-large",
        "texts": [
            "Hell World!",
            "Goodbye, World!"
        ]
     }'
```

##### Response

```json
{
    "model": "text-embedding-3-large",
    "embeddings": [
        [0.0222439254, -0.015302311, 0.001980035],
        [-0.015302311, 0.0233868278, 0.045129864]
    ]
}
```