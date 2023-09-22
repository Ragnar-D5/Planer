use std::fs::{create_dir, OpenOptions};
use iced::{Application, Element, Result, Settings, executor, Theme, Command, Subscription};
use iced::event::Event;
use iced::widget::container;

mod screen;
mod data;

use iced_core::Length;
use screen::calendar::{CalendarWidget, Calendar, self};
use data::{file_path, Date, };

#[derive(Clone)]
struct Planer {
    screen: Screen,
}

#[derive(Debug)]
pub enum Message {
    Calendar(calendar::Message),
    Event(Event),
}

#[derive(Clone)]
pub enum Screen {
    Calendar(calendar::CalendarWidget),
}

impl Application for Planer {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(flags: ()) -> (Planer, Command<Message>) {
        (Planer {screen: Screen::Calendar(CalendarWidget::new(Calendar { active_date: Date::now() }))}, Command::none())
    }

    fn title(&self) -> String {
        "Planer".to_string()
    }

    fn update (&mut self, message: Message) -> Command<Message>{
        match message {
            Message::Calendar(message) => {
                let Screen::Calendar(calendar) = &mut self.screen else {
                    return Command::none();
                };

                let command = calendar.update(message);

                command.map(Message::Calendar)

            }
            Message::Event(event) => {
                if let Screen::Calendar(calendar) = &mut self.screen {
                    return calendar.handle_event(event).map(Message::Calendar)
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::Calendar(calendar) => calendar.view().map(Message::Calendar)
        };

        container(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subs: Vec<iced::Subscription<Self::Message>> =
            vec![iced::subscription::events().map(|e| Message::Event(e))];

            iced::subscription::Subscription::batch(subs)
    }
}

impl Planer {
    
}

fn main() -> Result {
    let _ = create_dir(file_path());
    let mut path = file_path();
    path.push("saved.yml");
    let _file = OpenOptions::new()
        .create(true)  
        .write(true)
        .open(path);
    // let _appointments = YamlVec { data: vec![Appointment::default()]};
    // save_appointments(appointments);
    // let _saved = read_appointments();
    Planer::run(Settings {..Settings::default()})
}