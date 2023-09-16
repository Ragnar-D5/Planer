use iced::widget::{Text, Column, container};
use iced::widget::{row, button::Button, Container, column, container::Appearance};
use iced::{Element, Length, window, Command, Renderer, Rectangle};
use iced::advanced::{Widget};
use iced_core::{Size, Padding};
use crate::data::{Date, Appointment};


pub struct DayWidget {
    day: Date,
    content: Option<Vec<Appointment>>
}

pub struct CalendarWidget {
    start: Date,
    end: Date
}

#[derive(Debug, Clone)]
pub enum Message {}

impl DayWidget {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let button_increment = Button::new("Increment");
        let button_decrement = Button::new("Decrement");

        let content = row![]
            .push(button_increment)
            .push(button_decrement)
            .width(Length::Fill)
            .height(Length::Fill);

        content.into()
    }
}

impl CalendarWidget {

    pub fn new(start: Date, end: Date) -> Self {
        let mut days = vec![];
        for i in 0..30 {
            days.push(make_container(Appointment::default()))
        }
        CalendarWidget { start: Date::new(2023, Some(9), None, Some(1)), end: Date::new(2023, Some(10), None, Some(1)) } 
    }

    pub fn update(&mut self, message: Message) -> Command<Message>{

        Command::none()
    }
    
    pub fn view<'a>() -> Element<'a, Message> {
        let content = column![]
            .push(make_container_row())
            .push(make_container_row())
            .push(make_container_row())
            .push(make_container_row());
        content.into()
    }
}


fn make_container<'a>(appointment: Appointment) -> Element<'a, Message> {
    let content = column![]
        .push(Text::new("Placeholder"))
        .push(Text::new("Placeholder 2"));
    let mut container = Container::new(content);
    container = container.padding(Padding::new(30.0));
    container.into()
}

fn make_container_row<'a>() -> Element<'a, Message> {
    let mut content = row![];
    for i in 0..7 {
        content = content.push(make_container(Appointment::default()));
    }
    content.into()
}