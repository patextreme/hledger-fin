{ pkgs ? (import ../nix/input.nix).pkgs }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "hledger-fin";
  version = "0.1.0";
  src = ./.;
  cargoHash = "sha256-Q70m61BImxWEUFVxZYFpa1RT7tDzbsMtDPSZTGn2OXE=";
}
