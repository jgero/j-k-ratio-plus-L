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
        # dockerImage = pkgs.dockerTools.buildLayeredImage {
        #   name = "j-k-ratio-plus-L";
        #   contents = [ pkgs.glibc pkgs.gcc ];
        #   config = { Cmd = [ "${myRustBuild}/bin/j-k-ratio-plus-L" ]; };
        # };
      in {
        packages = {
          rustPackage = myRustBuild;
          # docker = dockerImage;
        };
        defaultPackage = myRustBuild;
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            lld
            cargo
          ];
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
      });
}
