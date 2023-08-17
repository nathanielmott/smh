mod retention;
mod subjects;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "smh")]
#[command(about = "smh: A tool for tracking how many times you've s'd your h", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// shake your head at something
    At { text: Vec<String> },
    /// find a specific entry
    Entry { name: Vec<String> },
    /// remove an entry
    Remove { name: Vec<String> },
    /// view the complete log
    Log,
}

fn main() -> eyre::Result<()> {
    retention::check_or_create_file()?;

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Entry { name }) => subjects::view_entry(name.join(" ")),
        Some(Commands::At { text }) => subjects::parse_subject(text),
        Some(Commands::Log) => subjects::view_log(),
        Some(Commands::Remove { name }) => subjects::remove_subject(name.join(" ")),
        None => subjects::view_log(),
    }
}
