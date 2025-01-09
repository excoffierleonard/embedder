# Step 1: Build the application
FROM rust:alpine AS builder

## Install dependencies
RUN apk add --no-cache \
    musl-dev \
    # openssl-dev \
    openssl-libs-static

## Create a new empty project
WORKDIR /app

## Copy only the manifests first
COPY Cargo.toml Cargo.lock ./
COPY embedder-web/embedder-core/Cargo.toml embedder-web/embedder-core/Cargo.toml
COPY embedder-web/Cargo.toml embedder-web/Cargo.toml

## Create dummy source files for caching dependencies
RUN mkdir src embedder-web/embedder-core/src embedder-web/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > embedder-web/embedder-core/src/lib.rs && \
    echo "pub fn dummy() {}" > embedder-web/src/lib.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs embedder-web/embedder-core/src/lib.rs embedder-web/src/lib.rs

## Now copy the real source code
COPY embedder-web/embedder-core/src embedder-web/embedder-core/src/
COPY embedder-web/src embedder-web/src/
COPY src src/

## Build the real application
RUN touch src/main.rs embedder-web/embedder-core/src/lib.rs embedder-web/src/lib.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM scratch

ENV OLLAMA_API_URL="http://ollama:11434/api/embed"
ENV OPENAI_API_KEY=""

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/embedder .

CMD ["./embedder"]