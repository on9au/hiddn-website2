# Build client
FROM node:slim as client-builder
WORKDIR /client-builder
COPY ./client .
RUN npm i && npm run build

# Build server
FROM rust:bookworm as server-builder
WORKDIR /server-builder
COPY . .
RUN rm -rf ./client
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates musl-dev && \
    rm -rf /var/lib/apt/lists/*
RUN cargo install typeshare-cli
RUN cargo build --release --bin hiddn-website --locked

# Copy stuff to final image
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && \
    apt-get install -y libssl3 libssl-dev openssl ca-certificates musl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    ldconfig
COPY --from=server-builder /server-builder/target/release/hiddn-website . 
COPY --from=client-builder /client-builder/dist/ ./static/
ENV RUST_LOG=info

# Set the entry point
CMD ["./hiddn-website"]