use crate::subjects::Subject;
use eyre::{Report, Result};
use serde::{Deserialize, Serialize};
use serde_json::{de, ser};
use std::collections::HashMap;
use std::fs::File;

pub fn check_existing_subjects() -> eyre::Result<HashMap<String, Subject>> {
    if let Ok(my_string) = std::fs::read_to_string("/home/mott/.smh") {
        let existing_subjects: HashMap<String, Subject> = serde_json::de::from_str(&my_string)?;
        Ok(existing_subjects)
    } else {
        let existing_subjects: HashMap<String, Subject> = HashMap::new();
        Ok(existing_subjects)
    }
}

pub fn write_to_file(list: HashMap<String, Subject>) -> eyre::Result<()> {
    let json_string = serde_json::ser::to_string(&list)?;
    let target_path = String::from("/home/mott/.smh");
    std::fs::write(&target_path, &json_string)?;
    Ok(())
}
