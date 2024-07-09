{ pkgs ? import <nixpkgs>, rustPlatform ? pkgs.rustPlatform, ... }:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
with pkgs; rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    openssl
  ];
}
