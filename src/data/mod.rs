pub mod date;
pub mod common;

pub use date::Date;
pub use common::file_path;

use std::fs::{read_to_string, OpenOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    High,
    Middle,
    Low
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appointment {
    pub id: i32,
    pub date: Date,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlVec {
    pub data: Vec<Appointment>
}

pub fn read_appointments() -> YamlVec {
    let mut path = file_path();
    path.push("saved.yml");
    let binding = read_to_string(path).unwrap();
    let file = binding.as_str();
    println!("{:?}", file);
    let scrape_config: YamlVec = serde_yaml::from_str(file).ok().unwrap();
    scrape_config
}

pub fn save_appointments(appointments: YamlVec) {
    let mut path = file_path();
    path.push("saved.yml");
    let yaml = serde_yaml::to_string(&appointments).unwrap();
    let file = OpenOptions::new()
        .write(true)    
        .open(path)
        .unwrap();
    println!("{:?}", yaml);
    serde_yaml::to_writer(file, &yaml).unwrap();
}