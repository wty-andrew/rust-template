FROM rust:1.93-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs

RUN cargo build --release

COPY . .

RUN touch ./src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/sandbox sandbox
