use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use crate::{assessment, engine, package, report, runner::Runner};

#[derive(Debug, Parser)]
#[command(
    name = "clilint",
    version,
    about = "Check a command-line interface against a behavioral conformance package",
    after_help = "Examples:\n  clilint check ./my-cli\n  clilint check ./my-cli --format json\n  clilint check ./my-cli --package ./team.toml"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run conformance checks against a target executable.
    Check(CheckArgs),
}

#[derive(Debug, clap::Args)]
struct CheckArgs {
    /// Path or command name of the executable to check.
    target: String,

    /// Load an additive extension package from a local TOML file.
    #[arg(long)]
    package: Option<PathBuf>,

    /// Attach a TOML or JSON AI-agent assessment. May be repeated.
    #[arg(long, action = clap::ArgAction::Append)]
    assessment: Vec<PathBuf>,

    /// Select human-readable or JSON output.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Default timeout for target invocations, in milliseconds.
    #[arg(long, default_value_t = 10_000)]
    timeout_ms: u64,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
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
    }
}

fn check(args: CheckArgs) -> Result<u8, String> {
    let package = package::load_resolved(args.package.as_deref())?;
    let mut runner = Runner::new(args.target.clone(), args.timeout_ms);
    let mut report = engine::check(&args.target, &package, &mut runner)?;

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
        let Command::Check(args) = cli.command;
        assert_eq!(args.assessment.len(), 2);
    }
}
