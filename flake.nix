{
  description = "Rust devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl # In case native SSL is used
              pkg-config

              # CI / Linting tools
              cargo-mutants
              cargo-hack
              cargo-msrv
              cargo-audit
              cargo-machete

              (rust-bin.stable.latest.default.override {
                extensions = [
                  "cargo"
                  "clippy"
                  "rust-src"
                  "rust-analyzer"
                ];
              })
            ];
          };
      }
    );
}
