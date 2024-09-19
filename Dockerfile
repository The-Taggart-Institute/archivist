FROM rust:latest as builder
RUN mkdir /archivist
WORKDIR /archivist
COPY . .
RUN rm -rf target
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/archivist /usr/local/bin/archivist
RUN mkdir /archivist
WORKDIR /archivist
COPY --from=builder /archivist/config.json /archivist/config.json
CMD ["archivist"]
