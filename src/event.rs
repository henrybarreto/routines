use crate::condition::Condition;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub name: String,
    pub condition: Condition,
    pub value: String,
    pub execute: String,
}

impl Event {
    pub fn load() -> Result<Vec<Event>, Error> {
        let mut config = File::open(Path::new("config.ron"))?;
        let mut data = String::new();
        config.read_to_string(&mut data);

        let events: Vec<Event> =
            ron::from_str(&data).expect("Could not convert the the config.ron file");

        return Ok(events);
    }
}
