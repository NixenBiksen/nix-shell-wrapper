{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoSha256 = "sha256-zJvFTcIkiZ/PL94zM4/FgjToREFHU3bOkQ0rFg3tDFE=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
