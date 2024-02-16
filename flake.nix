{
  description = "Kotlin to Java compression calculator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    templ = {
      url = "github:a-h/templ";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, templ, treefmt-nix, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      templ-bin = templ.packages.${system}.templ;
      treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
      # containerImage = pkgs.dockerTools.buildLayeredImage {
      #   name = "ghcr.io/jgero/j-k-ratio-plus-uppercase-l";
      #   tag = "latest";
      #   contents = with pkgs; [
      #     kotlin
      #     coreutils
      #     jd-cli
      #   ];
      #   maxLayers = 10;
      #   # config = { Cmd = [ "${myRustBuild}/bin/j-k-ratio-plus-L" "--production" "--static-path=${staticHtml}/build" ]; };
      # };
    in
    {
      formatter.${system} = treefmtEval.config.build.wrapper;
      checks.${system}.formatter = treefmtEval.config.build.check self;
      packages.${system}.default = pkgs.buildGoModule {
        pname = "j-k-ratio-plus-L";
        version = "2.0";
        # vendorHash = nixpkgs.lib.fakeHash;
        vendorHash = "sha256-FeGap2zXQCIFG894mUOHMDVLR34B84qfXZEGAF4ayjw=";
        src = ./.;
        nativeBuildInputs = [ templ-bin ];
        preBuild = ''
          templ generate hello.templ
        '';
      };
      # defaultPackage = myRustBuild;
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          go
          gopls
          templ-bin
          kotlin
          jd-cli
          nodejs
        ];
      };
    };
}
