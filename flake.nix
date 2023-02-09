{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustToolchain = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [
              "rust-src"
              "rust-analysis"
              "rustfmt"
              "clippy"
            ];
          };
          craneLib = crane.lib.${system}.overrideToolchain rustToolchain;
          src = craneLib.cleanCargoSource ./.;
          commonArgs = { inherit src; };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          my-crate = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });
      in
        {
        checks = {

          my-crate-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          my-crate-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          my-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          };
          packages.default = my-crate;
          apps.default = flake-utils.lib.mkApp {
            drv = my-crate;
          };
          devShells.default = pkgs.mkShell {
            inputsFrom = builtins.attrValues self.checks;
            nativeBuildInputs = with pkgs; [
              cargo
              clippy
              rust-analyzer
              rustc
              rustfmt
            ];
          };
        }
    );
}
