# chaseln
Chase symlinks!

## Development
- `nix develop --impure` to enter the devenv shell
    - See the [devenv flakes docs for more info](https://devenv.sh/guides/using-with-flakes/)
    - In the devenv shell, you can use `cargo` to build and test the project as if your system doesn't have nix
- `nix build .#chaseln` to build the project