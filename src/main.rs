use iced::{Application, Element, Result, Settings, executor, Theme, Command};

mod screen;
mod data;

use screen::calendar::{CalendarWidget, self};
use data::{file_path, save_appointments, read_appointments, YamlVec, Appointment, Date, Priority};

struct Planer {
    screen: Screen,
}

#[derive(Debug)]
pub enum Message {
    Calendar(calendar::Message),
}

pub enum Screen {
    Calendar(calendar::CalendarWidget),
    Settings,
}

impl Application for Planer {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(flags: ()) -> (Planer, Command<Message>) {
        (Planer {screen: Screen::Calendar(CalendarWidget::new(Date::default(), Date::default()))}, Command::none())
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
        }
    }

    fn view(&self) -> Element<Message> {
        CalendarWidget::view().map(Message::Calendar)
    }
}

fn main() -> Result{
    // let appointments = YamlVec { data: vec![Appointment::default()]};
    // save_appointments(appointments);
    // let saved = read_appointments();
    // println!("{:?}",saved.data[0]);
    Planer::run(Settings {..Settings::default()})
}