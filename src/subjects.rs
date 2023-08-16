use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub count: u8,
    pub reasons: Vec<String>,
}

impl Subject {
    pub fn new(name: &str, reason: &str) -> Subject {
        Self {
            name: name.to_string(),
            count: 1,
            reasons: vec![reason.to_string()],
        }
    }

    pub fn update(target: &mut Subject, reason: &str) -> Subject {
        let name = &target.name;
        let count: u8 = target.count + 1;
        target.reasons.push(reason.to_string());
        let reasons = &target.reasons;

        Self {
            name: name.to_string(),
            count,
            reasons: reasons.to_vec(),
        }
    }
}
