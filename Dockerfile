# syntax=docker/dockerfile:1

FROM rust:alpine AS dev

# Install system dependencies (for compilation)
RUN apk add --no-cache \
    pkgconfig \
    llvm-dev \
    clang-dev \
    openssl-dev \
    cmake \
    clang \
    curl \
    git \
    bash \
    build-base

# Create non-root user
RUN adduser -D devuser
USER devuser

# Set working directory
WORKDIR /app

# Cache dependencies early
COPY --chown=devuser:devuser Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy full project
RUN rm -rf src
COPY --chown=devuser:devuser . .

# Install cargo-watch
RUN cargo install cargo-watch

# Set env vars for development
ENV RUST_LOG=info

# Expose port if needed
EXPOSE 3000

CMD ["cargo", "watch", "-x", "run"]
