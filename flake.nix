{
  description = "Kotlin to Java compression calculator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        myRustBuild = rustPlatform.buildRustPackage {
          pname =
            "j-k-ratio-plus-L";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        staticHtml = pkgs.buildNpmPackage {
          name = "monaco-editor-frontend";
          src = ./static;
          npmDepsHash = "sha256-Fi01btSLEjBrBdAGCcXYD9/zDJOuVnevSVYm1iqffVs=";
        };
        containerImage = pkgs.dockerTools.buildLayeredImage {
          name = "ghcr.io/jgero/j-k-ratio-plus-uppercase-l";
          tag = "latest";
          contents = with pkgs; [
            kotlin
            coreutils
            jd-cli
          ];
          maxLayers = 5;
          config = { Cmd = [ "${myRustBuild}/bin/j-k-ratio-plus-L" "--production" "--static-path=${staticHtml}/lib/node_modules/j-k-ratio-plus-l" ]; };
        };
      in
      {
        packages = {
          rustPackage = myRustBuild;
          container = containerImage;
          frontend = staticHtml;
        };
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            kotlin
            jd-cli
            nodejs
          ];
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}
