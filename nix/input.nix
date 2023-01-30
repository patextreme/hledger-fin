let sources = import ./sources.nix;
in rec {
  pkgs = import sources.nixpkgs {
    config = { allowUnfree = true; };
    overlays = [ (import sources.nixpkgs-mozilla) ];
  };
}
