# Use the official Rust image as a base
FROM rust:1.83.0 as builder

# Set the working directory
WORKDIR /usr/src/app

# Install Node.js and npm
ENV NODE_VERSION=20.18.0
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash && \
    export NVM_DIR="$HOME/.nvm" && \
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
    nvm install ${NODE_VERSION} && \
    nvm use v${NODE_VERSION} && \
    nvm alias default v${NODE_VERSION} && \
    node --version && \
    npm --version

# Install typeshare-cli
RUN cargo install typeshare-cli

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the Rust and client code
RUN export NVM_DIR="$HOME/.nvm" && \
    [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" && \
    cd client && npm install && npm run build && \
    export SQLX_OFFLINE=true && \
    cd .. && cargo build --release

# Use the same base image for the final image to ensure compatibility
FROM rust:1.83.0

# Set the working directory
WORKDIR /usr/src/app

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built files from the builder stage
COPY --from=builder /usr/src/app/target/release/hiddn-website /usr/src/app/target/release/hiddn-website
COPY --from=builder /usr/src/app/client/dist /usr/src/app/client/dist
COPY --from=builder /usr/src/app/client/server-ssr.js /usr/src/app/client/server-ssr.js
COPY --from=builder /usr/src/app/client/package.json /usr/src/app/client/package.json
COPY --from=builder /usr/src/app/client/node_modules /usr/src/app/client/node_modules

# Set the entry point
CMD ["./target/release/hiddn-website"]