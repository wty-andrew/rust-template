export DATABASE_URL := "postgresql://postgres:postgres@localhost:5432"

[private]
default:
  @just --list --unsorted

# Create the database
create-db:
  @sqlx database create

# Create a new migration
add-migration name:
  @sqlx migrate add {{name}}

# Apply all pending migrations
run-migrations:
  @sqlx migrate run

# Drop the database
drop-db:
  @sqlx database drop

# Shorthand for create database and then run migrations
setup-db:
  @sqlx database setup

# Drop and recreate the database, then run migrations
reset-db:
  @sqlx database reset -f

# Save query metadata for offline verification
prepare-db:
  @cargo sqlx prepare --check

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
