{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoHash = "sha256-K1mSJ+6v975RG2p9jVncnMdF7uvt/eEY8wqtdfSaZ64=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
