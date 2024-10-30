# chaseln
Chase symlinks!

![Example chasing a symlink generated using home-manager's mkOutOfStoreSymlink](https://github.com/user-attachments/assets/ea067faf-6cd3-4c95-9dbf-4a4576214345)

## Development
- `nix develop --impure` to enter the devenv shell
    - See the [devenv flakes docs for more info](https://devenv.sh/guides/using-with-flakes/)
    - In the devenv shell, you can use `cargo` to build and test the project as if your system doesn't have nix
- `nix build .#chaseln` to build the project
