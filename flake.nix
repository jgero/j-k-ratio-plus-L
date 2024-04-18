{
  description = "Kotlin to Java compression calculator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    templ = {
      url = "github:a-h/templ/v0.2.543";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, templ, devenv, treefmt-nix, ... } @ inputs:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      templ-bin = templ.packages.${system}.templ;
      treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
    in
    {
      formatter.${system} = treefmtEval.config.build.wrapper;
      checks.${system}.formatter = treefmtEval.config.build.check self;
      packages.${system} = rec {
        goMod = pkgs.buildGoModule {
          pname = "j-k-ratio-plus-L";
          version = "2.0";
          # vendorHash = nixpkgs.lib.fakeHash;
          vendorHash = "sha256-FeGap2zXQCIFG894mUOHMDVLR34B84qfXZEGAF4ayjw=";
          src = ./.;
          nativeBuildInputs = [ templ-bin ];
          preBuild = ''
            templ generate
          '';
        };
        container = pkgs.dockerTools.buildLayeredImage {
          name = "ghcr.io/jgero/j-k-ratio-plus-uppercase-l";
          tag = "latest";
          contents = with pkgs; [
            kotlin
            coreutils
            jd-cli
          ];
          maxLayers = 10;
          config = {
            Cmd = [ "${goMod}/bin/${goMod.pname}" ];
            Env = [ "KOTLIN_BIN=${pkgs.kotlin}/bin/kotlinc" "JD_BIN=${pkgs.jd-cli}/bin/jd-cli" ];
          };
        };
        default = pkgs.writeScriptBin "wrapped-mod" ''
          KOTLIN_BIN="${pkgs.kotlin}/bin/kotlinc" JD_BIN="${pkgs.jd-cli}/bin/jd-cli" ${goMod}/bin/${goMod.pname}
        '';
      };
      devShells.${system}.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          {
            languages.go.enable = true;
            packages = with pkgs; [
              reflex
              templ-bin
              kotlin
              jd-cli
              nodejs
            ];

            env.KOTLIN_BIN = "${pkgs.kotlin}/bin/kotlinc";
            env.JD_BIN = "${pkgs.jd-cli}/bin/jd-cli";

            scripts.dev-server.exec = ''
              reflex -R '_templ.go$' -s -- sh -c 'templ generate && go run .'
            '';
          }
        ];
      };
    };
}
