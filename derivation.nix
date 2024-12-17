{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-fOE0T7p68Ie27rMGDthJ5jXA8B5tYhWntX7x6FFmNEY=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
