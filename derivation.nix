{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-WM2T9vcBRGNVridxwaQwwveidZtV2w/9NejasCIJpLA=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
