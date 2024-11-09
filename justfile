dev:
  @cargo watch -x check -x test -x run

lint:
  @cargo clippy

fmt:
  @cargo fmt

check:
  @cargo check

build:
  @cargo build

test:
  @DATABASE_URL=postgresql://postgres:postgres@localhost:5432 cargo test

run:
  @cargo run

clean:
  @cargo clean
