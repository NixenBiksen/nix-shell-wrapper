{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-Q+Pbv+aL5P1CZ93dXW9hWRv7wNDdqNDuV5AhXQMUG/8=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
