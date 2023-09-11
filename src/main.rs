use iced::{Sandbox, Element, Result, Settings};

mod screen;

use screen::calendar::{Calendar, self};

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
    Planer::run(Settings {..Settings::default()})
}