use crate::retention;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub count: u8,
    pub reasons: HashMap<String, String>,
}

impl Subject {
    pub fn new(name: &str, reason: &str) -> Subject {
        let now: DateTime<Local> = Local::now();
        let date_string: String = format!("{}", now.format("%b %d, %Y %H:%M"));
        Self {
            name: name.to_string(),
            count: 1,
            reasons: HashMap::from([(date_string, reason.to_string())]),
        }
    }

    pub fn update(target: &mut Subject, reason: &[String]) -> Subject {
        let now: DateTime<Local> = Local::now();
        let date_string: String = format!("{}", now.format("%b %d, %Y %H:%M"));
        let name = &target.name;
        let count: u8 = target.count + 1;
        target
            .reasons
            .insert(date_string, reason.to_owned().join(" "));
        let reasons = &target.reasons;

        Self {
            name: name.to_string(),
            count,
            reasons: reasons.to_owned(),
        }
    }
}

pub fn view_log() -> eyre::Result<()> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".smh");

    let file_string = std::fs::read_to_string(&path)?;

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

pub fn view_entry(name: String) -> eyre::Result<()> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".smh");

    let file_string = std::fs::read_to_string(&path)?;

    let subjects: HashMap<String, Subject> = serde_json::de::from_str(&file_string)?;
    if subjects.contains_key(&name) {
        let entry = subjects.get(&name).unwrap();
        println!(
            "Name: {}, Count: {}, Reasons: {:?}",
            entry.name,
            entry.reasons.len(),
            entry.reasons
        );
        Ok(())
    } else {
        println!("Error: couldn't find an entry with that name");
        Ok(())
    }
}

pub fn parse_subject(args: Vec<String>) -> eyre::Result<()> {
    if let Some(index) = args.iter().position(|r| r == "for") {
        let (left, right) = args.split_at(index);
        let name = &left[0..];
        let reason = &right[1..];
        add_to_list(name, &reason)?;
        Ok(())
    } else {
        println!("Error: please input a reason");
        Ok(())
    }
}

pub fn remove_subject(name: String) -> eyre::Result<()> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".smh");

    let file_string = std::fs::read_to_string(&path)?;

    let mut subjects: HashMap<String, Subject> = serde_json::de::from_str(&file_string)?;
    if subjects.contains_key(&name) {
        subjects.remove_entry(&name);
        retention::write_to_file(subjects)?;
        Ok(())
    } else {
        println!("Error: couldn't find that entry");
        Ok(())
    }
}

fn add_to_list(name: &[String], reason: &[String]) -> eyre::Result<()> {
    // std::fs::File::create("/home/mott/.smh")?;
    let mut existing_subjects: HashMap<String, Subject>;

    if let Ok(subjects_list) = retention::check_existing_subjects() {
        existing_subjects = subjects_list;
    } else {
        existing_subjects = HashMap::new();
    }

    if existing_subjects.contains_key(&name.join(" ")) {
        let target_subject = existing_subjects.get_mut(&name.join(" ")).unwrap();
        Subject::update(target_subject, reason);
    } else {
        let new_subject = Subject::new(
            &name.join(" ").to_string().to_lowercase(),
            &reason.to_owned().join(" "),
        );
        existing_subjects.insert(name.join(" ").to_string(), new_subject);
    }

    retention::write_to_file(existing_subjects)?;
    Ok(())
}
