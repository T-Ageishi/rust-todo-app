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

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /workspace/target/release/api_server /app/api_server

EXPOSE 8080
USER nonroot:nonroot

CMD ["/app/api_server"]