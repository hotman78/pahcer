pub(crate) mod runner;
pub(crate) mod settings;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    /// Initialize the project
    Init(settings::InitArgs),
    /// Run tests
    Run(runner::RunArgs),
}

fn main() {
    let args = Cli::parse();
    dbg!(&args);

    if let Err(e) = run_command(args) {
        eprintln!("{}", format!("Error: {:?}", e).yellow().bold());
        std::process::exit(1);
    }
}

fn run_command(args: Cli) -> Result<(), anyhow::Error> {
    Ok(match args.command {
        Command::Init(args) => {
            settings::gen_setting_file(&args);
            let settings = settings::load_setting_file()?;
            dbg!(settings);
        }
        Command::Run(args) => {
            runner::run(args)?;
        }
    })
}
