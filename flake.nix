{
  description = "Kotlin to Java compression calculator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
      let
        overlays = [ (import rust-overlay) ];
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit overlays system; };
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
          src = ./.;
          npmDepsHash = "sha256-d5BSZ2yOQhOuSTkvOuWxiscvXHJnXjC+52tYxiyDl1Y=";
          installPhase = ''
            mkdir -p $out
            mv build $out
          '';
        };
        containerImage = pkgs.dockerTools.buildLayeredImage {
          name = "ghcr.io/jgero/j-k-ratio-plus-uppercase-l";
          tag = "latest";
          contents = with pkgs; [
            kotlin
            coreutils
            jd-cli
          ];
          maxLayers = 10;
          config = { Cmd = [ "${myRustBuild}/bin/j-k-ratio-plus-L" "--production" "--static-path=${staticHtml}/build" ]; };
        };
      in
      {
        packages.${system} = {
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
      };
}
