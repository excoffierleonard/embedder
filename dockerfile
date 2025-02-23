# Step 1: Build the application
FROM rust:alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static

WORKDIR /app

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

# Step 2: Create final image
FROM alpine

ENV OLLAMA_API_URL="http://ollama:11434/api/embed"
ENV OPENAI_API_KEY=""

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/embedder-bin .

EXPOSE 8080

CMD ["./embedder-bin"]
