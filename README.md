# `nix-shell-wrapper`: A flake-based wrapper for `nix-shell`

`nix-shell-wrapper` is a simple tool designed to simplify working with `nix-shell` in a flake-based setup. It provides a convenient wrapper that helps you quickly enter development shells with `ns <someProgram>`. While it stays fairly minimal, it adds some helpful features like automatic prompt updates to reflect the current shell environment.

If you're already using nix flakes and want a streamlined way to launch shells and customize your shell environments, this tool might be a good fit for you.

## Key Features

- **Flakes support**: Integrates with the nix flake system.
- **Improved UX**: A small wrapper that updates your shell prompt with environment information.
- **Customizable**: Easily modify and extend your shell environment with custom packages or specific versions of `nixpkgs`.

---

## Installation & Setup

`nix-shell-wrapper` has exclusively been tested on NixOS systems based on flakes, though it should work in other setups. If you get it working with other setups, feel free to update this documentation.

### 1. Add `nix-shell-wrapper` as a flake input

To use `nix-shell-wrapper` in your NixOS system, add it as a flake input in your `flake.nix` file:

```nix
{
  inputs = {
    # ...
    nix-shell-wrapper = {
        url = "github:NixenBiksen/nix-shell-wrapper";
        # Optionally make it follow your existing nixpkgs or flake-utils references.
        inputs.nixpkgs.follows = "nixpkgs";
        inputs.flake-utils.follows = "flake-utils";
    };
    # ...
  };
}
```

### 2. Define the `ns` shell function

To easily invoke the tool, add the following function to your shell configuration (`~/.bashrc`, `~/.zshrc`, etc.). The easiest way to do this is probably through home-manager:

```bash
function ns() {
    history -a # Save command history before starting the shell
    ${nix-shell-wrapper}/bin/nix-shell-wrapper "$@"
    history -r # Reload command history after exiting the shell
}
```

Once this function is in place, you can launch a nix shell by simply typing `ns <package>`.

### 3. (Optional) Update your shell prompt

For better visibility of your current nix environments, you can customize your prompt. This example gives you a simple prompt that conditionally adds `$NIX_SHELL_WRAPPER_DESCRIPTIONS` if it is present. You probably want to customize it though:

```bash
# Example prompt to show how to optionally include the $NIX_SHELL_WRAPPER_DESCRIPTIONS if present
CYAN="\[\e[0;36m\]"
NC="\[\e[0m\]"
PS1="\${NIX_SHELL_WRAPPER_DESCRIPTIONS:+${CYAN}\$NIX_SHELL_WRAPPER_DESCRIPTIONS${NC}:}"
PS1="${PS1}\w$ "
```

### 4. (Optional) Customize your package list

You can extend or customize the list of packages available when using `nix-shell-wrapper`. Here's how to include specific packages or modify the version of `nixpkgs` used.

#### Step 4.1: Modify your flake output

In your system `flake.nix`, adjust the `nix-shell-wrapper-pkgs` output to include custom packages or tweak the default environment:

```nix
{
  outputs = { flake-utils, nixpkgs, ... }@flakeArgs:
    flake-utils.lib.eachDefaultSystem (system: {
      # ...

      # Include everything from pkgs and selfPackages
      nix-shell-wrapper-pkgs."${system}".default = pkgs // selfPackages;

      # Or specify the individual packages you want to include
      nix-shell-wrapper-pkgs."${system}".default = {
        inherit (hello) pkgs;
        myPackage = selfPackages.myPackage;
      };

      # You can also use package prefixes, requiring you to write `ns pkgs.<program>`
      nix-shell-wrapper-pkgs."${system}".default = {
        inherit pkgs;
      };
    });
}
```

#### Step 4.2: Set the `$NIX_SHELL_WRAPPER_FLAKE` variable

To get nix-shell-wrapper to actually use the packages defined in `nix-shell-wrapper-pkgs`, you need to set the `$NIX_SHELL_WRAPPER_FLAKE` to point to the flake that has the `nix-shell-wrapper-pkgs` output. Typically this should point to your system flake. One way of doing that would be something like this:

```nix
# In your flake.nix
{
  outputs = { flake-utils, nixpkgs, ... }@flakeArgs:
    flake-utils.lib.eachDefaultSystem (system: {
      # ...
    });
}
# Somewhere inside a nix module:
{
  # You need access to the flakeArgs variable by passing it into the nix module
  nix.registry.self.flake = flakeArgs.self;
  environment.variables."NIX_SHELL_WRAPPER_FLAKE" = "self";
}
```

---

## Environment Variables

- `NIX_SHELL_WRAPPER_FLAKE`: This optional variable allows you to specify a flake reference, giving you control over which packages are accessible within the shell. You can for instance set it to a flake in your registry or to an absolute path.
- `NIX_SHELL_WRAPPER_DESCRIPTIONS`: This variable holds a description of the current Nix shell environment. You can display this in your shell prompt for better context when working across multiple shells.
- `NIX_SHELL_WRAPPER_SYSTEM`: This is an internal variable. It is only used during the build process to specify the system architecture and is set via `derivation.nix`. You don't need to interact with it directly, but is listed here for completion.

---

## Contributing

If you encounter any issues or have ideas for new features, feel free to open a pull request or issue on the GitHub repository.

---

## License

This project is licensed under the [MIT License](./LICENSE).
