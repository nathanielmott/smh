mod retention;
mod subjects;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "smh")]
#[command(about = "to track how many times you've s'd your h", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Entry { name: Vec<String> },
    At { text: Vec<String> },
    Log,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Entry { name }) => subjects::view_entry(name.join(" ")),
        Some(Commands::At { text }) => subjects::parse_subject(text),
        Some(Commands::Log) => subjects::view_log(),
        None => subjects::view_log(),
    }
}
