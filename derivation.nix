{
  rustPlatform,
  nix-gitignore,
  system,
}:

rustPlatform.buildRustPackage {
  pname = "nix-shell-wrapper";
  version = "0.1.0";
  src = nix-gitignore.gitignoreSource [ "*.nix" ] ./.;
  cargoSha256 = "sha256-lqbuYIq7NAPVMqiAt70YMPNkGF3ogMIXxTKGcT28rRY=";
  NIX_SHELL_WRAPPER_SYSTEM = system;
}
