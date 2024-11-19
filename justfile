default:
  @just -l

flash:
  @espflash flash target/riscv32imc-esp-espidf/debug/sandbox

open-monitor:
  @espflash monitor

lint:
  @cargo clippy

fmt:
  @cargo fmt

check:
  @cargo check

build:
  @cargo build

test:
  @cargo test

run:
  @cargo run

clean:
  @cargo clean
