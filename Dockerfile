FROM rust:latest

# Installer les dépendances système
RUN apt-get update -qq && apt-get install -y -qq \
    build-essential \
    gcc-mingw-w64 \
    curl \
    pkg-config \
    zip \
    git \
    nodejs \
    libssl-dev \
    && apt-get clean

# Installer les outils Rust
RUN rustup component add clippy rustfmt \
    && rustup target add x86_64-pc-windows-gnu \
    && cargo install git-cliff cargo-release