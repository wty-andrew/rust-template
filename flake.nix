{
  inputs = {
    nix-ros-overlay.url = "github:lopsided98/nix-ros-overlay/develop";

    nixpkgs.follows = "nix-ros-overlay/nixpkgs";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nix-ros-overlay/nixpkgs";
    };
  };

  outputs = { self, nix-ros-overlay, nixpkgs, rust-overlay }:
    let
      allSystems = [ "x86_64-linux" "aarch64-linux" ];

      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f (
        import nixpkgs {
          inherit system;
          overlays = [
            nix-ros-overlay.overlays.default
            (import rust-overlay)
          ];
        }
      ));
    in
    {
      devShells = forAllSystems (pkgs: with pkgs;
        let
          rosEnv = (with rosPackages.jazzy; buildEnv {
            paths = [
              ament-cmake-core
              cargo-ament-build
              (colcon.withExtensions [
                python3Packages.colcon-cargo
                python3Packages.colcon-ros-cargo
              ])
              python-cmake-module
              ros-base
            ];
          });
        in
        {
          default = mkShell {
            nativeBuildInputs = [
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              rustPlatform.bindgenHook
              rosEnv
            ];

            buildInputs = [
              rust-analyzer
            ];
          };
        });
    };
}
