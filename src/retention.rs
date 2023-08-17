use crate::subjects::Subject;
use dirs;
use eyre;
use serde_json::{de, ser};
use std::collections::HashMap;

pub fn check_existing_subjects() -> eyre::Result<HashMap<String, Subject>> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".smh");

    if let Ok(my_string) = std::fs::read_to_string(&path) {
        let existing_subjects: HashMap<String, Subject> = de::from_str(&my_string)?;
        Ok(existing_subjects)
    } else {
        let existing_subjects: HashMap<String, Subject> = HashMap::new();
        Ok(existing_subjects)
    }
}

pub fn write_to_file(list: HashMap<String, Subject>) -> eyre::Result<()> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".smh");

    let json_string = ser::to_string(&list)?;
    std::fs::write(&path, &json_string)?;
    Ok(())
}
