use std::{env, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

use crate::{
    assessment, engine,
    project_config::{self, LoadOptions},
    report,
    runner::Runner,
};

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

    /// Attach a TOML or JSON AI-agent assessment. May be repeated.
    #[arg(long, action = clap::ArgAction::Append)]
    assessment: Vec<PathBuf>,

    /// Select human-readable or JSON output.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Default timeout for target invocations, in milliseconds.
    #[arg(long, default_value_t = 10_000)]
    timeout_ms: u64,

    /// Do not use the network to restore missing check bundles.
    #[arg(long)]
    offline: bool,

    /// Require project declarations to match the project lockfile.
    #[arg(long)]
    locked: bool,
}

#[derive(Debug, clap::Args)]
struct BundleArgs {
    #[command(subcommand)]
    command: BundleCommand,
}

#[derive(Debug, Subcommand)]
enum BundleCommand {
    /// Record and install a source, or install all missing declarations.
    Install {
        /// Local path or Git URL. Add #ref and ::path for a Git subdirectory.
        source: Option<String>,
        /// Do not use the network.
        #[arg(long)]
        offline: bool,
        /// Require declarations to match the lockfile.
        #[arg(long)]
        locked: bool,
    },
    /// Resolve requested Git refs to exact commits.
    Lock { name: Option<String> },
    /// List project check-bundle declarations and installation state.
    List {
        /// Write one JSON document.
        #[arg(long)]
        json: bool,
    },
    /// Re-resolve requested Git refs and install the result.
    Update { name: Option<String> },
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
    let bundle = project_config::load_for_check(
        &root,
        args.check_bundle.as_deref(),
        LoadOptions {
            offline: args.offline,
            locked: args.locked,
        },
    )?;
    let mut runner = Runner::new(args.target.clone(), args.timeout_ms);
    let mut report = engine::check(&args.target, &bundle, &mut runner)?;

    for path in &args.assessment {
        let document = assessment::load(path)?;
        assessment::attach(&mut report, document)?;
    }
    report.recalculate();

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
        BundleCommand::Install {
            source,
            offline,
            locked,
        } => {
            let statuses =
                project_config::install(&root, source.as_deref(), LoadOptions { offline, locked })?;
            print_bundle_statuses(&statuses);
        }
        BundleCommand::Lock { name } => {
            let lock = project_config::lock(&root, name.as_deref())?;
            println!(
                "locked {} check bundle(s) in .clilint/lock.toml",
                lock.check_bundles.len()
            );
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
        BundleCommand::Update { name } => {
            let statuses = project_config::update(&root, name.as_deref())?;
            print_bundle_statuses(&statuses);
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
        let resolved = status
            .resolved_commit
            .as_deref()
            .map(|commit| format!(" at {commit}"))
            .unwrap_or_default();
        println!("{}: {state}{resolved}", status.name);
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
