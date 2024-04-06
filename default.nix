{ pkgs ? import <nixpkgs>, rustPlatform ? pkgs.rustPlatform, ... }:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
