{ pkgs }:

rec {
  rust = pkgs.latest.rustChannels.stable.rust;
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
}
