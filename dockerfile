# Step 1: Build the application
FROM rust:alpine AS builder

## Install dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev

## Create a new empty project
WORKDIR /app

## Copy only the manifests first
COPY Cargo.toml Cargo.lock ./

## Create dummy source files for caching dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs

## Now copy the real source code
COPY src src/

## Build the real application
RUN touch src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM scratch

ENV OPENAI_API_KEY=""

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/embedder .

CMD ["./embedder"]