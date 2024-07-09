{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      allSystems = [ "x86_64-linux" "aarch64-linux" ];

      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f (
        import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        }
      ));
    in
    {
      devShells = forAllSystems (pkgs: with pkgs; {
        default = mkShell {
          nativeBuildInputs = [
            pkg-config
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          ];

          buildInputs = [
            openssl
            rust-analyzer
          ];

          OTEL_SERVICE_NAME = "sandbox";
        };
      });

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./default.nix { };
      });
    };
}
