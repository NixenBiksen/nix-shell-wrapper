# `nix-shell-wrapper`: An opinionated wrapper around nix-shell using flakes

## How to use

```bash
CYAN="\[\e[0;36m\]"
NC="\[\e[0m\]"
PS1="\${NIX_SHELL_WRAPPER_DESCRIPTIONS:+${CYAN}\$NIX_SHELL_WRAPPER_DESCRIPTIONS${NC}:} $"

function ns() {
  history -a
  ${nix-shell-wrapper}/bin/nix-shell-wrapper "$@"
  history -r
}
```

TODO: Write some text about how this works

## Environment variables

### `NIX_SHELL_WRAPPER_FLAKE`

This variable can optionally be set to a flake reference. Set this if you want additional control over which packages you access to when using the wrapper. You can for instance write something like this in the `flake.nix` for your system:

```nix
      nix-shell-wrapper-pkgs."${system}".default = pkgs // {
        myPackage = self.myPackage;
      };
```

And then add something like this to a NixOS module somewhere:

```nix
nix.registry.self.flake = flakeInputArguments.self;
environment.variables."NIX_SHELL_WRAPPER_FLAKE" = "self";
```

TODO: Document where `flakeInputArguments` come from.

### `NIX_SHELL_WRAPPER_DESCRIPTIONS`

This variable contains a human-readable description of the shells you are inside. You can e.g. put it in your `$PS1` when using bash to easily be able to see which shells you are inside.

### `NIX_SHELL_WRAPPER_SYSTEM`

This variable is set automatically by `derivation.nix` while building the application. It is neither set nor read at runtime. You will not need to care about it, but we're including it here for completeness.
