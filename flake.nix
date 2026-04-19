{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem =
        { pkgs, system, ... }:
        with pkgs;
        let
          nativeBuildInputs = [
            clang
            lld
            pkg-config
            ((rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            wasm-bindgen-cli_0_2_108 # need to match the version used by bevy
          ];

          # https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md#nix
          buildInputs = [
            alsa-lib
            vulkan-loader
            vulkan-tools
            libudev-zero
            libx11
            libxcursor
            libxi
            libxrandr
            libxkbcommon
            wayland
          ];
        in
        {
          # https://github.com/hercules-ci/flake-parts/issues/106
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          packages.default = callPackage ./default.nix {
            inherit nativeBuildInputs buildInputs;
          };

          devShells.default = mkShell {
            inherit nativeBuildInputs;
            buildInputs = buildInputs ++ [ rust-analyzer ];

            LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          };
        };
    };
}
