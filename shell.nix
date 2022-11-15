{ pkgs ? import <nixpkgs> {} }:

with pkgs;
let
  elixir = elixir_1_14;
in
mkShell {
  buildInputs = [ elixir ];

  shellHook = ''
    mix local.hex --if-missing
  '';
}
