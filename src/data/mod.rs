pub mod date;
pub mod common;

pub use date::Date;
pub use common::file_path;
use std::fmt;

use std::fs::{read_to_string, OpenOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    High,
    Middle,
    Low
}

impl Priority {
    pub const ALL: &[Self] = &[Self::High, Self::Middle, Self::Low];
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::High => "High",
            Priority::Middle => "Middle",
            Priority::Low => "Low",
        }
        .fmt(f)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Appointment {
    pub id: i32,
    pub date: Date,
    pub priority: Priority,
    pub warning: Date,
    pub tags: Option<Vec<String>>,
    pub description: String,
}

impl Default for Appointment {
    fn default() -> Self {
        Appointment { id: 0, date: Date::default(), priority: Priority::Low, warning: Date::default(), tags: None, description: "Description".to_string() }
    }
}

impl Appointment {
    pub fn description(&self) -> &String {
        &self.description
    }
}

pub fn read_appointments() -> Vec<Appointment> {
    let mut path = file_path();
    path.push("saved.yml");
    let binding = read_to_string(path).unwrap();
    let file = binding.as_str();
    let scrape_config: Vec<Appointment> = serde_yaml::from_str(file).ok().unwrap();
    scrape_config
}

pub fn save_appointments(appointments: Vec<Appointment>) {
    let mut path = file_path();
    path.push("saved.yml");
    let file = OpenOptions::new()
        .create(true)
        .write(true)    
        .open(path)
        .unwrap();
    serde_yaml::to_writer(file, &appointments).unwrap();
}