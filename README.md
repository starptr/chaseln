# chaseln
Chase symlinks!

![Example chasing a symlink generated using home-manager's mkOutOfStoreSymlink](https://github.com/user-attachments/assets/ea067faf-6cd3-4c95-9dbf-4a4576214345)
- Example chasing a symlink generated using home-manager's mkOutOfStoreSymlink

## Development
- `nix develop --impure` to enter the devenv shell
    - See the [devenv flakes docs for more info](https://devenv.sh/guides/using-with-flakes/)
    - In the devenv shell, you can use `cargo` to build and test the project as if your system doesn't have nix
- `nix build .#chaseln` to build the project
    - The build env and the dev env are coincidentally the same (because they're both using packages from the `nixpkgs` flake input),
    but they can be different if you set the version in eg. `devenv.nix` without also changing the build env.