{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    systems.url = "github:nix-systems/default";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      systems,
    }:
    flake-utils.lib.eachSystem (import systems) (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages = flake-utils.lib.flattenTree {
          nix-shell-wrapper = pkgs.callPackage ./derivation.nix { inherit system; };
        };
      }
    );
}
