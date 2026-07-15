{
  description = "A CLI utility for automatic stitching of digital negatives";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = [
            pkgs.hugin
            pkgs.libraw.dev
          ];
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
        };
        # Build only the dependencies once so they can be reused across the
        # package build and every check.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        rawGlue = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
        rawGlueWrapped = pkgs.symlinkJoin {
          name = "raw-glue";
          paths = [ rawGlue ];
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = ''
            wrapProgram $out/bin/raw-glue \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.hugin ]}
          '';
        };
      in
      {
        checks = {
          inherit rawGlue;
          rawGlue-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });
          rawGlue-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });
          rawGlue-fmt = craneLib.cargoFmt {
            inherit src;
          };
          rawGlue-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
            cargoNextestExtraArgs = "--no-tests=pass";
          });
        };
        packages.default = rawGlueWrapped;
        apps.default = flake-utils.lib.mkApp {
          drv = rawGlueWrapped;
        };
        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [
            pkgs.rust-analyzer
          ];
        };
      }
    );
}
