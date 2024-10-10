# `nix-shell-wrapper`: An opinionated wrapper around nix-shell using flakes

## How to use

### 1. Add this repo as a flake input to your system

It can look something like this:

```nix
nix-shell-wrapper = {
  url = "github:NixenBiksen/nix-shell-wrapper";
  inputs.nixpkgs.follows = "nixpkgs";
  inputs.flake-utils.follows = "flake-utils";
};
```

### 2. Add a shell alias

It could look something like this in your bash config:

```bash
function ns() {
  history -a
  ${nix-shell-wrapper}/bin/nix-shell-wrapper "$@"
  history -r
}
```

### 3. (Optional) Add the shell descriptions to your shell prompt

It could look something like this:

```bash
CYAN="\[\e[0;36m\]"
NC="\[\e[0m\]"
PS1="\${NIX_SHELL_WRAPPER_DESCRIPTIONS:+${CYAN}\$NIX_SHELL_WRAPPER_DESCRIPTIONS${NC}:} $"
```

### 4. (Optional) Customize the import experience

If you want to e.g. use a specific nixpkgs or include additional packages, you need to do two things. First add an additional flake output to your flake:

```nix
nix-shell-wrapper-pkgs."${system}".default = pkgs // selfPackages;

# Alternatively you could customize it by doing something like this:
nix-shell-wrapper-pkgs."${system}".default = {
  inherit pkgs;
  myPackage = selfPackages.myPackage;
};
```

Then you set the `$NIX_SHELL_WRAPPER_FLAKE` environment variable to a valid flake. You could look something like this:

```nix
# Here flakeInputArguments is a reference to the arguments passed to
# the input function at the top-level of the flake. This way we put a
# reference to the current flake into the flake registry.
nix.registry.self.flake = flakeInputArguments.self;
environment.variables."NIX_SHELL_WRAPPER_FLAKE" = "self";
```

## Environment variables

### `NIX_SHELL_WRAPPER_FLAKE`

This variable can optionally be set to a flake reference. Set this if you want additional control over which packages you access to when using the wrapper. You can for instance write something like this in the `flake.nix` for your system:

### `NIX_SHELL_WRAPPER_DESCRIPTIONS`

This variable contains a human-readable description of the shells you are inside. You can e.g. put it in your `$PS1` when using bash to easily be able to see which shells you are inside.

### `NIX_SHELL_WRAPPER_SYSTEM`

This variable is set automatically by `derivation.nix` while building the application. It is neither set nor read at runtime. You will not need to care about it, but we're including it here for completeness.
