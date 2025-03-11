# Start with a Rust base image
FROM rust:1.81-slim-bookworm as builder

# Create a new empty shell project
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl

# Copy over your manifests
COPY Cargo.toml Cargo.lock ./

# This is a trick to pre-build dependencies
# Create a dummy main.rs to build dependencies
RUN mkdir -p src
RUN echo "fn main() {println!(\"dummy\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy in your actual source code
COPY src ./src

# Build for release
RUN touch src/main.rs
RUN cargo build --release

# Final stage: the runtime image
FROM debian:bookworm-slim

# Accept build argument for the decryption key
ARG DOTENV_PRIVATE_KEY
ENV DOTENV_PRIVATE_KEY=${DOTENV_PRIVATE_KEY}

# Create app directory
WORKDIR /app

# Install SSL certificates for HTTPS requests and curl for dotenvx
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Install dotenvx
RUN curl -sfS https://dotenvx.sh/install.sh | sh

# Copy the binary and .env file
COPY --from=builder /usr/src/app/target/release/backend /app/api-service
COPY .env /app/.env

# Set environment variables - these can be overridden at runtime
ENV SERVER_ADDRESS=0.0.0.0:7878

# Expose the port
EXPOSE 7878

# Use dotenvx to load environment variables and run the application
CMD dotenvx run -- ./api-service