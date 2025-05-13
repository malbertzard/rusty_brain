{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustPackages.rustc
    pkgs.rustPackages.cargo
    pkgs.fasm
  ];
}
