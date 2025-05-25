{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-z+Wkf5tpKkvXUwm9aaLbC1ZJ4p7dQ8moWXF/W0na0vU=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
