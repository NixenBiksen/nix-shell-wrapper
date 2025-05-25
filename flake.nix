{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        nix-shell-wrapper = pkgs.callPackage ./derivation.nix { inherit system; };
      in
      {
        checks = {
          inherit nix-shell-wrapper;
        };
        packages = rec {
          inherit nix-shell-wrapper;
          default = nix-shell-wrapper;
        };
      }
    );
}
