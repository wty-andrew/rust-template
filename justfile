set dotenv-load

lint:
  @cargo clippy

fmt:
  @cargo fmt

check:
  @cargo check

build:
  @cargo build

clean:
  @cargo clean

copy: build
  @scp {{justfile_directory()}}/target/aarch64-unknown-linux-gnu/debug/sandbox $TARGET_HOST:$TARGET_PATH

start-gdbserver:
  @ssh $TARGET_HOST "nohup gdbserver :${TARGET_PORT} ${TARGET_PATH}"
