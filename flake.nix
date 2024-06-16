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
          config.allowUnfree = true;
        }
      ));
    in
    {
      devShells = forAllSystems (pkgs: with pkgs; {
        default = mkShell.override { stdenv = gcc12Stdenv; } {
          buildInputs = [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            rust-analyzer
            cudaPackages.cudnn
          ];

          CUDA_ROOT = "${cudatoolkit}";
          CUDNN_LIB = "${cudaPackages.cudnn}";
        };
      });

      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./default.nix { };
      });
    };
}
