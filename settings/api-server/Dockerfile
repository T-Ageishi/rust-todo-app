FROM rust:1.92-bookworm AS builder

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY api-server/Cargo.toml api-server/Cargo.toml

RUN mkdir -p api-server/src \
    && echo "fn main() {}" > api-server/src/main.rs \
    && cargo build --release -p api_server \
    && rm -rf api-server/src

COPY . .
RUN cargo build --release -p api_server

FROM debian:bookworm-slim
RUN useradd -r -u 10001 nonroot

WORKDIR /app

COPY --from=builder /workspace/target/release/api_server /app/api_server
COPY settings/api-server/run.sh /run.sh
RUN chmod +x /run.sh \
 && chown nonroot:nonroot /run.sh /app/api_server

EXPOSE 8080
USER nonroot:nonroot

RUN chmod +x /run.sh
ENTRYPOINT ["/run.sh"]
