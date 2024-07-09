FROM rust:1.83-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs

RUN cargo build --release

COPY . .

RUN touch ./src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/sandbox server

COPY config config

ENV APP_ENV production

CMD [ "/app/server", "--host", "0.0.0.0", "--port", "80" ]
