{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    {
      self,
      nixpkgs,
      devenv,
      systems,
      ...
    }@inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config = {
              allowBroken = true;
            };
          };
          metadata = builtins.fromTOML (builtins.readFile ./app/Cargo.toml);
        in
        {
          devenv-up = self.devShells.${system}.default.config.procfileScript;
          chaseln = pkgs.rustPackages.rustPlatform.buildRustPackage {
            pname = metadata.package.name;
            version = metadata.package.version;
            src = ./app;
            cargoLock = {
              lockFile = ./app/Cargo.lock;
            };
          };
          default = self.packages.${system}.chaseln;
        }
      );

      devShells = forEachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [ (import ./devenv.nix) ];
          };
        }
      );

      overlays = {
        chaseln = final: prev: {
          chaseln = self.packages.${final.stdenv.hostPlatform.system}.chaseln;
        };
        default = self.overlays.chaseln;
      };

      formatter = forEachSystem (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        pkgs.nixfmt-rfc-style
      );
    };
}
