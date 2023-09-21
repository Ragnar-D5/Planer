use std::default;

use iced::widget::{Text, Column, container};
use iced::widget::{row, button::Button, Container, column, container::{Appearance, Id}};
use iced::{Element, Length, window, Command, Renderer, Rectangle};
use iced::advanced::{Widget};
use iced_core::{Size, Padding, Pixels};
use crate::data::{Date, Appointment};


pub struct DayWidget {
    day: Date,
    content: Option<Vec<Appointment>>
}

#[derive(Copy, Clone)]
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
        CalendarWidget { start: Date::new(2023, Some(9), None, Some(1)), end: Date::new(2023, Some(10), None, Some(1)) } 
    }

    pub fn update(&mut self, message: Message) -> Command<Message>{

        Command::none()
    }
    
    pub fn view<'a>(window_size: (u32, u32)) -> Element<'a, Message> {
        let content = column![]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .push(make_container_row(window_size))
            .push(make_container_row(window_size))
            .push(make_container_row(window_size))
            .push(make_container_row(window_size));
        content.into()
    }
}

pub fn make_container<'a>(window_size: (u32, u32), appointment: Appointment) -> Element<'a, Message> {
    // let padding = Padding::from([window_size.1 as f32 / 8.0, window_size.0 as f32 / 14.0]);
    let content = column![]
        .push(Button::new("Hello").width(Length::Fill))
        .push(Button::new("other hello").width(Length::Fill));
    let mut container = Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .style(DayContainer::new().move_to_style())
        .into();

    container
}

pub fn make_container_row<'a>(window_size: (u32, u32)) -> Element<'a, Message> {
    let mut content = row![]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);
    for i in 0..7 {
        content = content.push(make_container( window_size, Appointment::default()));
    }
    content
        .into()
}

#[derive(Default)]
enum DayContainerStyle {
    #[default]
    Bordered,
}

pub struct DayContainer(DayContainerStyle);

impl Default for DayContainer {
    fn default() -> Self {
        Self(DayContainerStyle::Bordered)
    }
}

impl DayContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_to_style(self) -> iced::theme::Container {
        self.into()
    }
}

impl std::convert::From<DayContainer> for iced::theme::Container {
    fn from(value: DayContainer) -> Self {
        iced::theme::Container::Custom(Box::new(value))
    }
}

impl iced::widget::container::StyleSheet for DayContainer {
    type Style = iced::theme::Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance { 
            text_color: Some(style.palette().text),
            background: Some(iced::Color::TRANSPARENT.into()), 
            border_radius: 6.0.into(), 
            border_width: 2.0.into(), 
            border_color: iced::Color {a: 0.5, ..style.palette().text} 
        }
    }
}