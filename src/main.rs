mod string_truncation;

use clap::{
    builder::{styling::AnsiColor, Styles},
    command, Parser,
};
use color_eyre::{
    eyre::{self, WrapErr},
    Result,
};
use std::{
    os::unix::prelude::CommandExt,
    path::{Path, PathBuf},
};
use string_truncation::truncate_string;

const NIX_SHELL_WRAPPER_SYSTEM: &'static str = env!("NIX_SHELL_WRAPPER_SYSTEM");

fn make_pretty(path: &Path) -> Result<String> {
    let path = std::fs::canonicalize(path)
        .wrap_err_with(|| format!("Unable to canonicalize {}", path.display()))?;
    let home_dir = dirs::home_dir().ok_or_else(|| eyre::eyre!("Could not get home directory"))?;

    let mut in_home = false;
    let path = if let Ok(p) = path.strip_prefix(home_dir) {
        in_home = true;
        p
    } else {
        &path
    };
    let mut components = path
        .components()
        .filter_map(|c| match c {
            std::path::Component::RootDir => None,
            std::path::Component::Normal(p) => Some(p.to_string_lossy().into_owned()),
            _ => unimplemented!(),
        })
        .collect::<Vec<_>>();

    let mut pretty = format!("/{}", components.pop().unwrap());
    for parent in components.into_iter().rev() {
        let new = format!("/{}{}", parent, pretty);
        if new.len() < 25 {
            pretty = new;
        } else {
            return Ok(format!("…{}", pretty));
        }
    }
    if in_home {
        Ok(format!("~{}", pretty))
    } else {
        Ok(pretty)
    }
}

fn clap_v3_styling() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug)]
#[command(styles = clap_v3_styling())]
#[command(about = r#"nix-shell-wrapper: Easy tool for entering nix-shells

If no arguments are given, the tool will try to evaluate a flake.nix or shell.nix in the current directory.

 The default subcommand is the exprs subcommand. In other words these commands do the same:

* `nix-shell-wrapper hello`
* `nix-shell-wrapper exprs hello`"#)]
enum App {
    /// Enters a shell based on a shell.nix
    Shell(ShellCommand),

    /// Enters a shell based on a devshell in a flake.nix
    Flake(FlakeCommand),

    /// Enters a shell based on a derivation file. This is a convenience over using callPackage manually
    Derivation(DerivationCommand),

    /// Enters a shell based on a number of nix expressions
    Exprs(ExprsCommand),

    #[command(external_subcommand)]
    /// A bit of a hack to make the Exprs subcommand work without specifying the exprs keyword
    ExprsExternal(Vec<String>),
}

#[derive(Parser, Debug)]
struct ShellCommand {
    /// Path to a `shell.nix`
    #[clap(default_value = "./shell.nix")]
    shell: PathBuf,
}

#[derive(Parser, Debug)]
struct FlakeCommand {
    /// Path to a directory with a flake.nix which contains a devshell
    #[clap(default_value = ".")]
    flake: PathBuf,
}

#[derive(Parser, Debug)]
struct DerivationCommand {
    /// Path to a the derivation
    derivation: String,

    /// Arguments to give to the derivation
    #[clap(default_value = "{}")]
    args: String,
}

#[derive(Parser, Debug)]
struct ExprsCommand {
    /// Extra flake paths to include. Use the form `--flake=flakeName=PATH`
    #[arg(long, value_parser=parse_flake_pair)]
    flake: Vec<(String, PathBuf)>,
    /// A number of nix expressions; each expression must evaluate to a package
    exprs: Vec<String>,
}

fn parse_flake_pair(s: &str) -> Result<(String, PathBuf), String> {
    if let Some((left, right)) = s.split_once('=') {
        Ok((
            left.to_string(),
            PathBuf::from(right)
                .canonicalize()
                .map_err(|e| format!("Cannot canonicalize path {right:?}: {e:?}"))?,
        ))
    } else {
        Err(format!("Cannot parse {s:?} as a flake+path pair"))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = App::parse();

    let mut env = Vec::new();
    let mut cmd;
    match args {
        App::Shell(ShellCommand { shell }) => {
            cmd = std::process::Command::new("nix-shell");
            env.push(make_pretty(&shell)?);
            cmd.arg(shell);
        }
        App::Flake(FlakeCommand { flake }) => {
            cmd = std::process::Command::new("nix");
            env.push(make_pretty(&flake)?);
            cmd.arg("develop");
            cmd.arg(flake);
        }
        App::Derivation(DerivationCommand { derivation, args }) => {
            cmd = std::process::Command::new("nix");
            cmd.args(["shell", "--impure", "--expr"]);

            let mut combined_expr = expr_prefix(Vec::new());
            combined_expr.push_str("(callPackage ");
            combined_expr.push_str(&derivation);
            combined_expr.push(' ');
            combined_expr.push_str(&args);
            combined_expr.push_str(")]");
            cmd.arg(combined_expr);
            env.push(truncate_string(&derivation));
        }
        App::Exprs(ExprsCommand { flake, exprs }) => {
            cmd = make_exprs_command(exprs, flake, &mut env)
        }
        App::ExprsExternal(exprs) => {
            cmd = make_exprs_command(exprs, Vec::new(), &mut env);
        }
    }
    let env = format!("{}", env.join("+"));
    cmd.env(
        "NIX_SHELL_WRAPPER_DESCRIPTIONS",
        match std::env::var("NIX_SHELL_WRAPPER_DESCRIPTIONS") {
            Ok(prev) => format!("{} {}", prev, env),
            Err(_) => env,
        },
    );
    cmd.exec();

    Ok(())
}

fn make_exprs_command(
    exprs: Vec<String>,
    extra_flakes: Vec<(String, PathBuf)>,
    env: &mut Vec<String>,
) -> std::process::Command {
    let mut cmd = std::process::Command::new("nix");
    cmd.args(["shell", "--impure", "--expr"]);

    let mut combined_expr = expr_prefix(extra_flakes);

    for expr in exprs {
        combined_expr.push('(');
        combined_expr.push_str(&expr);
        combined_expr.push_str(") ");
        env.push(truncate_string(&expr));
    }

    combined_expr.push(']');
    cmd.arg(combined_expr);
    cmd
}

fn expr_prefix(extra_flakes: Vec<(String, PathBuf)>) -> String {
    let mut let_exprs = Vec::new();
    let mut with_exprs = Vec::new();

    if let Ok(flake) = std::env::var("NIX_SHELL_WRAPPER_FLAKE") {
        let_exprs.push(format!("  systemFlake = builtins.getFlake {flake:?};"));
        with_exprs.push(format!(
            "systemFlake.nix-shell-wrapper-pkgs.{NIX_SHELL_WRAPPER_SYSTEM:?}.default"
        ));
    } else {
        let_exprs.push(format!(r#"  nixpkgs = builtins.getFlake "nixpkgs";"#));
        let_exprs.push(format!(
            "  pkgs = import nixpkgs {{ system = {NIX_SHELL_WRAPPER_SYSTEM:?}; }};"
        ));
        with_exprs.push(format!("pkgs"));
    }

    for (name, path) in extra_flakes {
        let_exprs.push(format!("  {name} = builtins.getFlake {path:?};"));
    }

    let mut expr = "[".to_string();

    if !with_exprs.is_empty() {
        expr = format!("with {}; {expr}", with_exprs.join(" "));
    }

    if !let_exprs.is_empty() {
        expr = format!("let\n{}in\n{expr}", let_exprs.join("\n"));
    }

    expr
}
