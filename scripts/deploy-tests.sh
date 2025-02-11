#!/bin/bash

# Fail on any error
set -e

# Source .env file if it exists
if [ -f .env ]; then
    echo "Loading environment from .env file..."
    source .env
else
    echo "Warning: .env file not found"
fi

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Base URL
BASE_URL="http://embedder.excoffierleonard.com"

# Function to print test results
print_result() {
    local test_name=$1
    local result=$2
    if [ $result -eq 0 ]; then
        echo -e "${GREEN}✓ $test_name passed${NC}"
    else
        echo -e "${RED}✗ $test_name failed${NC}"
        exit 1
    fi
}

# Function to test API endpoint with a provided model
test_embed_endpoint() {
    local test_name=$1
    local model=$2
    local headers=$3

    echo -e "\nTesting embedding with $test_name..."

    local response
    local http_code

    # Use eval to properly handle the headers string
    response=$(eval "curl -s -w '%{http_code}' $headers \
        --url '$BASE_URL/embed' \
        --header 'Content-Type: application/json' \
        --data '{
            \"model\": \"$model\",
            \"texts\": [
                \"Hello World!\",
                \"Goodbye, World!\"
            ]
        }'"
    )

    # Extract HTTP code (last 3 characters)
    http_code="${response: -3}"
    # Extract response body (everything except last 3 characters)
    response_body="${response:0:${#response}-3}"

    # Check if response contains expected fields
    if [ "$http_code" == "200" ] && echo "$response_body" | grep -q "embeddings"; then
        print_result "$test_name" 0
    else
        echo -e "${RED}Expected successful response with embeddings${NC}"
        echo -e "${RED}Got HTTP code: $http_code${NC}"
        echo -e "${RED}Response body: $response_body${NC}"
        print_result "$test_name" 1
    fi
}

# Function to test API endpoint without providing a model
test_embed_endpoint_no_model() {
    local test_name="No model provided (default to Ollama)"
    echo -e "\nTesting embedding with $test_name..."

    local response
    local http_code

    response=$(curl -s -w '%{http_code}' \
        --url "$BASE_URL/embed" \
        --header "Content-Type: application/json" \
        --data '{
            "texts": [
                "Hello World!",
                "Goodbye, World!"
            ]
        }'
    )

    # Extract HTTP code (last 3 characters)
    http_code="${response: -3}"
    # Extract response body (everything except last 3 characters)
    response_body="${response:0:${#response}-3}"

    if [ "$http_code" == "200" ] && echo "$response_body" | grep -q "embeddings"; then
        print_result "$test_name" 0
    else
        echo -e "${RED}Expected successful response with embeddings${NC}"
        echo -e "${RED}Got HTTP code: $http_code${NC}"
        echo -e "${RED}Response body: $response_body${NC}"
        print_result "$test_name" 1
    fi
}

echo "Starting deployment tests..."

# Test 1: Ollama request (model provided, non-OpenAI)
test_embed_endpoint "Ollama embedding" "nomic-embed-text" ""

# Test 2: OpenAI request with custom auth
if [ -z "$OPENAI_API_KEY" ]; then
    echo -e "${RED}Warning: OPENAI_API_KEY not set, skipping OpenAI auth test${NC}"
else
    test_embed_endpoint "OpenAI with auth" "nomic-embed-text" "--header 'Authorization: Bearer \$OPENAI_API_KEY'"
fi

# Test 3: OpenAI fallback (expected to fail)
echo -e "\nTesting OpenAI fallback (expected to fail)..."
response=$(curl -s -w '%{http_code}' \
    --url "$BASE_URL/embed" \
    --header "Content-Type: application/json" \
    --data '{
        "model": "text",
        "texts": [
            "Hello World!",
            "Goodbye, World!"
        ]
    }'
)

# Extract HTTP code and body
http_code="${response: -3}"
response_body="${response:0:${#response}-3}"

if [ "$http_code" == "500" ] || [ "$http_code" == "400" ]; then
    print_result "OpenAI fallback test (expected failure)" 0
else
    echo -e "${RED}Expected failure response${NC}"
    echo -e "${RED}Got HTTP code: $http_code${NC}"
    echo -e "${RED}Response body: $response_body${NC}"
    print_result "OpenAI fallback test (expected failure)" 1
fi

# Test 4: No model provided - should default to Ollama embedding
test_embed_endpoint_no_model

echo -e "\nAll tests completed!"
