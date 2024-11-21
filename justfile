[private]
default:
  @just --list --unsorted

dev:
  @cargo watch -x 'run --features=bevy/dynamic_linking'

lint:
  @cargo clippy

fmt:
  @cargo fmt

check:
  @cargo check

build:
  @cargo build --features=bevy/dynamic_linking

build-wasm:
  @cargo build --profile wasm-release --target wasm32-unknown-unknown
  @wasm-bindgen \
    --out-name sandbox \
    --out-dir web \
    --target web \
    target/wasm32-unknown-unknown/wasm-release/sandbox.wasm

test:
  @cargo test

run:
  @cargo run --features=bevy/dynamic_linking

run-example $name:
  @cargo run --features=bevy/dynamic_linking --example $name

clean:
  @cargo clean
