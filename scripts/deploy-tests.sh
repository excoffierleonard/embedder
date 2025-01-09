#!/bin/bash

# Fail on any error
set -e

# Test curl
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