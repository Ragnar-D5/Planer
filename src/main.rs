use std::fs::{create_dir, OpenOptions};

use iced::futures::io::Copy;
use iced::{Application, Element, Result, Settings, executor, Theme, Command, Pixels, Subscription};
use iced::event::Event;
use iced::widget::{row, column, Text, Container};

mod screen;
mod data;

use screen::calendar::{CalendarWidget, Calendar, self};
use data::{file_path, save_appointments, read_appointments, YamlVec, Appointment, Date, Priority};

#[derive(Copy, Clone)]
struct Planer {
    screen: Screen,
    calendar: Calendar,
    window_size: (u32, u32),
}

#[derive(Debug)]
pub enum Message {
    Calendar(calendar::Message),
    Event(Event),
}

#[derive(Copy, Clone)]
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
        (Planer {screen: Screen::Calendar(CalendarWidget::new(Date::default(), Date::default())), window_size: (1000,1000), calendar: Calendar {active_date: Date::now()}}, Command::none())
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
                if let Event::Window(iced::window::Event::Resized { width, height }) = event {
                    self.window_size = (width, height);
                    println!("{:?}", self.window_size);
                    self.view();
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        CalendarWidget::view(self.calendar).map(Message::Calendar)
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