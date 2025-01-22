{
  inputs = {
    nixpkgs.url = "nixpkgs";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-parts,
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      perSystem = {
        lib,
        system,
        self',
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        packages = {
          default = self'.packages.theme;
          theme = pkgs.callPackage ./package.nix {
            inherit (pkgs) lib buildNpmPackage;
          };
        };
        devShells.default = with pkgs;
          mkShell {
            nativeBuildInputs = [];

            buildInputs = [
              nodejs_23
            ];

            LD_LIBRARY_PATH = lib.makeLibraryPath [];
          };

        formatter = pkgs.alejandra;
      };
    };
}
