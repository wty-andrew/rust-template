{ lib
, rustPlatform
, nativeBuildInputs
, buildInputs
, ...
}:
let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  inherit buildInputs nativeBuildInputs;

  postInstall = ''
    cp -r assets $out/bin/
  '';
}
