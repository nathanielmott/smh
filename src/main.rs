mod retention;
mod subjects;

use crate::subjects::Subject;
use clap::{Parser, Subcommand};
use dirs;
use serde_json::de;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(name = "smh")]
#[command(about = "to track how many times you've s'd your h", long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    reason: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Log,
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    let name = cli.name;
    let reason = cli.reason;

    match cli.command {
        Some(Commands::Log) => view_log(),
        None => add_to_list(
            name.expect("provide a name!"),
            reason.expect("provide a reason!"),
        ),
    }
}

fn view_log() -> eyre::Result<()> {
    let file_string = std::fs::read_to_string("/home/mott/.smh")?;
    let subjects: HashMap<String, Subject> = serde_json::de::from_str(&file_string)?;
    let list = subjects.into_values();
    for subject in list {
        println!(
            "Name: {}, Count: {}, Reasons: {:?}",
            subject.name,
            subject.reasons.len(),
            subject.reasons
        );
    }
    Ok(())
}

fn add_to_list(name: String, reason: String) -> eyre::Result<()> {
    // std::fs::File::create("/home/mott/.smh")?;
    let mut existing_subjects: HashMap<String, Subject>;

    if let Ok(subjects_list) = retention::check_existing_subjects() {
        existing_subjects = subjects_list;
    } else {
        existing_subjects = HashMap::new();
    }

    if existing_subjects.contains_key(&name) {
        let target_subject = existing_subjects.get_mut(&name).unwrap();
        Subject::update(target_subject, &reason);
        // existing_subjects.insert(name.to_string(), target_subject);
    } else {
        let new_subject = subjects::Subject::new(&name.to_string(), &reason.to_string());
        existing_subjects.insert(name.to_string(), new_subject);
    }

    retention::write_to_file(existing_subjects)?;
    Ok(())
}
