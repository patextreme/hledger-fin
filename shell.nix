{ pkgs ? (import ./nix/input.nix).pkgs }:

let
  baseDir = toString ./.;
  common = pkgs.callPackage ./nix/common.nix { };
in pkgs.mkShell {
  packages = with pkgs; [ common.rust ];
  shellHook = ''
  '';
}
