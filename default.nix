{ lib, pkgs, rustPlatform, ... }:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [
    pkg-config
    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ];

  buildInputs = with pkgs; [
    openssl
  ];

  doCheck = false; # require database connection

  SQLX_OFFLINE = true;
}
