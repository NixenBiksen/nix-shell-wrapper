{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoSha256 = "sha256-eLhCoFsbX9Oncm+GOWf+X9Cg7a2f6k7EEe35rEPFXmw=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
