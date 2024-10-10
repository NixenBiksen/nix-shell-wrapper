mod string_truncation;

use clap::{
    builder::{styling::AnsiColor, Styles},
    command, CommandFactory, Parser,
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
struct App {
    #[clap(short = 'f')]
    /// Path to a `shell.nix`
    shell: Option<PathBuf>,

    #[clap(long = "flake")]
    /// Path to a `flake.nix` which contains a devshell
    flake: Option<PathBuf>,

    /// A list of expression to be evaluated
    exprs: Vec<String>,
}

fn main() -> Result<()> {
    let mut env = Vec::new();
    color_eyre::install()?;

    let mut args = App::parse();

    if args.shell.is_some() && !args.exprs.is_empty() {
        use std::io::Write;
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();
        writeln!(stderr, "Cannot use both paths and exprs").ok();
        writeln!(stderr).ok();
        App::command().print_help().ok();
        writeln!(stderr).ok();
        return Ok(());
    }

    if args.shell.is_none() && args.exprs.is_empty() && args.flake.is_none() {
        let shell_path = PathBuf::from("shell.nix");
        let flake_path = PathBuf::from("flake.nix");
        if shell_path.exists() {
            args.shell = Some(shell_path);
        } else if flake_path.exists() {
            args.flake = Some(".".into());
        } else {
            eyre::bail!("Cannot find a nix shell to run");
        }
    }

    let mut cmd;
    if let Some(shell) = args.shell {
        cmd = std::process::Command::new("nix-shell");
        env.push(make_pretty(&shell)?);
        cmd.arg(shell);
    } else if let Some(flake) = args.flake {
        cmd = std::process::Command::new("nix");
        env.push(make_pretty(&flake)?);
        cmd.arg("develop");
        cmd.arg(flake);
    } else {
        cmd = std::process::Command::new("nix");
        cmd.args(["shell", "--impure", "--expr"]);

        let mut combined_expr;
        if let Ok(system_flake) = std::env::var("NIX_SHELL_WRAPPER_SYSTEM_FLAKE") {
            combined_expr = format!(
                r#"
                    let
                        systemFlake = builtins.getFlake {system_flake:?};
                    in
                        with systemFlake.nix-shell-wrapper-pkgs.{NIX_SHELL_WRAPPER_SYSTEM:?}.default; [
                "#
            );
        } else {
            combined_expr = format!(
                r#"
                    let
                        nixpkgs = builtins.getFlake "nixpkgs";
                        pkgs = import nixpkgs {{ system = {NIX_SHELL_WRAPPER_SYSTEM:?}; }};
                    in
                        with pkgs; [
                "#
            );
        }

        for expr in args.exprs {
            combined_expr.push('(');
            combined_expr.push_str(&expr);
            combined_expr.push_str(") ");
            env.push(truncate_string(&expr));
        }

        combined_expr.push(']');
        cmd.arg(combined_expr);
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
