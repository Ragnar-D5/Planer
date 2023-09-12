use iced::{Sandbox, Element, Result, Settings};

mod screen;
mod data;

use screen::calendar::{Calendar, self};
use data::{file_path, save_appointments, read_appointments, YamlVec, Appointment, Date, Priority};

struct Planer;

#[derive(Debug)]
enum Message {
    Calendar(calendar::Message)
}

pub enum Screen {
    Calendar(calendar::Calendar)
}

impl Sandbox for Planer {
    type Message = Message;

    fn new() -> Self {
        Planer
    }

    fn title(&self) -> String {
        "Planer".to_string()
    }

    fn update (&mut self, message: Message) {
        match message {
            Message::Calendar(message) => {}
        }
    }

    fn view(&self) -> Element<Message> {
        Calendar.view().map(Message::Calendar)
    }
}

fn main() -> Result{
    let appointments = YamlVec {
        data: vec![ Appointment {
            id: 0,
            date: Date {
                year: 2023,
                month: Some(9),
                day: Some(12),
                week: None
            },
            priority: Priority::High,
            tags: vec!["tag".to_string()],
            description: "hello".to_string()
        }]
    };
    save_appointments(appointments);
    let saved = read_appointments();
    println!("{:?}",saved.data[0]);
    Planer::run(Settings {..Settings::default()})
}