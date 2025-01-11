{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];
      perSystem = { pkgs, system, ... }: with pkgs;
        rec {
          # https://github.com/hercules-ci/flake-parts/issues/106
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          packages.default = callPackage ./default.nix { };

          devShells.default = mkShell {
            inputsFrom = [ packages.default ];

            buildInputs = [
              rust-analyzer
            ];

            packages = [
              sqlx-cli
            ];

            OTEL_SERVICE_NAME = "sandbox";
            OTEL_EXPORTER_OTLP_PROTOCOL = "grpc";
            OTEL_EXPORTER_OTLP_ENDPOINT = "http://localhost:5081";
            # basic auth header: `echo -n <email>@<password> | base64`
            OTEL_EXPORTER_OTLP_HEADERS = "Authorization=Basic YWRtaW5AZG9tYWluLmNvbTphZG1pbg==,organization=default,stream-name=default";
          };
        };
    };
}
