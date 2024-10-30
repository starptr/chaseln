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

  outputs = { self, nixpkgs, devenv, systems, ... } @ inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: let
          pkgs = import nixpkgs {
            inherit system;
            config = { allowBroken = true; };
          };
        in
        {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        chaseln = pkgs.rustPackages.rustPlatform.buildRustPackage {
          pname = "chaseln";
          version = "0.1.0";
          src = ./app;
          cargoLock = {
            lockFile = ./app/Cargo.lock;
          };
        };
      });

      devShells = forEachSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            default = devenv.lib.mkShell {
              inherit inputs pkgs;
              modules = [
                (import ./devenv.nix)
              ];
            };
          });
    };
}
