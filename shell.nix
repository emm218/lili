{ pkgs ? import <nixpkgs> {} }:

with pkgs;
let
  elixir = beam.packages.erlangR24.elixir_1_12;
in
mkShell {
  buildInputs = [ elixir ];

  shellHook = ''
    mix local.hex
    mix archive.install hex phx_new 1.6.15
  '';
}
