FROM mcr.microsoft.com/devcontainers/base:ubuntu

# ---- System deps ----
RUN apt-get update && apt-get install -y \
    curl \
    clang \
    lld \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ---- Node.js ----
ARG NODE_VERSION=20
RUN curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | bash - \
    && apt-get install -y nodejs

# ---- Rust ----
USER vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN rustup target add wasm32-unknown-unknown

# ---- wasm-pack ----
RUN cargo install wasm-pack

# ---- Useful Rust tools (optional but nice) ----
RUN cargo install cargo-watch

WORKDIR /workspaces
