{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-aID5CsDLD2ddw2X+Mc/MwaY9OiOJsCtGENcT/cHTYac=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
