use std::{env, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

use crate::{assessment, engine, project_config, report, runner::Runner};

#[derive(Debug, Parser)]
#[command(
    name = "clilint",
    version,
    about = "Check a command-line interface against behavioral checks",
    after_help = "Examples:\n  clilint check ./my-cli\n  clilint check ./my-cli --format json\n  clilint check ./my-cli --check-bundle ./team.toml"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run conformance checks against a target executable.
    Check(CheckArgs),
    /// Install and manage project check bundles.
    Bundle(BundleArgs),
}

#[derive(Debug, clap::Args)]
struct CheckArgs {
    /// Path or command name of the executable to check.
    target: String,

    /// Load an additive check bundle from a local TOML file.
    #[arg(long)]
    check_bundle: Option<PathBuf>,

    /// Supply a JSON Assessment. May be repeated.
    #[arg(long, action = clap::ArgAction::Append)]
    assessment: Vec<PathBuf>,

    /// Select human-readable or JSON output.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Default timeout for target invocations, in milliseconds.
    #[arg(long, default_value_t = 10_000)]
    timeout_ms: u64,
}

#[derive(Debug, clap::Args)]
struct BundleArgs {
    #[command(subcommand)]
    command: BundleCommand,
}

#[derive(Debug, Subcommand)]
enum BundleCommand {
    /// Record and validate a local check-bundle path.
    Install {
        /// Local path to a check-bundle directory or TOML file.
        source: PathBuf,
    },
    /// List project check-bundle declarations and installation state.
    List {
        /// Write one JSON document.
        #[arg(long)]
        json: bool,
    },
    /// Remove one project check-bundle declaration.
    Remove { name: String },
}

#[derive(Clone, Copy, Debug, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
enum OutputFormat {
    Human,
    Json,
}

pub fn run() -> Result<u8, String> {
    run_from(Cli::parse())
}

fn run_from(cli: Cli) -> Result<u8, String> {
    match cli.command {
        Command::Check(args) => check(args),
        Command::Bundle(args) => bundle(args),
    }
}

fn check(args: CheckArgs) -> Result<u8, String> {
    let root =
        env::current_dir().map_err(|error| format!("could not read current directory: {error}"))?;
    let bundle = project_config::load_for_check(&root, args.check_bundle.as_deref())?;
    let mut runner = Runner::new(args.target.clone(), args.timeout_ms);
    let assessments = args
        .assessment
        .iter()
        .map(|path| assessment::load(path))
        .collect::<Result<Vec<_>, _>>()?;
    let report = engine::check(&args.target, &root, &bundle, &mut runner, &assessments)?;

    match args.format {
        OutputFormat::Human => print!("{}", report::human(&report)),
        OutputFormat::Json => println!("{}", report::json(&report)?),
    }

    Ok(if report.has_failures() { 1 } else { 0 })
}

fn bundle(args: BundleArgs) -> Result<u8, String> {
    let root =
        env::current_dir().map_err(|error| format!("could not read current directory: {error}"))?;
    match args.command {
        BundleCommand::Install { source } => {
            let statuses = project_config::install(&root, &source)?;
            print_bundle_statuses(&statuses);
        }
        BundleCommand::List { json } => {
            let statuses = project_config::list(&root)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&statuses).map_err(|error| error.to_string())?
                );
            } else {
                print_bundle_statuses(&statuses);
            }
        }
        BundleCommand::Remove { name } => {
            project_config::remove(&root, &name)?;
            println!("removed check bundle {name}");
        }
    }
    Ok(0)
}

fn print_bundle_statuses(statuses: &[project_config::BundleStatus]) {
    if statuses.is_empty() {
        println!("No project check bundles are installed.");
        return;
    }
    for status in statuses {
        let state = if status.installed {
            "installed"
        } else {
            "missing"
        };
        println!("{}: {state}", status.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn command_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn assessment_option_is_repeatable() {
        let cli = Cli::try_parse_from([
            "clilint",
            "check",
            "target",
            "--assessment",
            "one.toml",
            "--assessment",
            "two.json",
        ])
        .unwrap();
        let Command::Check(args) = cli.command else {
            panic!("expected check command");
        };
        assert_eq!(args.assessment.len(), 2);
    }
}
