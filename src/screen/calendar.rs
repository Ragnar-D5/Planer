use std::default;

use iced::widget::{Text, Column, container};
use iced::widget::{row, button::Button, Container, column, container::{Appearance, Id}};
use iced::{Element, Length, window, Command, Renderer, Rectangle};
use iced::advanced::{Widget};
use iced_core::mouse::ScrollDelta;
use iced_core::{Size, Padding, Pixels};
use crate::data::{Date, Appointment};

#[derive(Copy, Clone)]
pub struct Calendar {
    pub active_date: Date,
}

#[derive(Copy, Clone)]
pub struct CalendarWidget {
    active_date: Date,
}

#[derive(Debug, Clone)]
pub enum Message {
    TimeIncrement,
    TimeDecrement,
}

impl CalendarWidget{

    pub fn new(calendar: Calendar) -> Self {
        CalendarWidget { active_date: Date::now() }
    }

    pub fn update(&mut self, message: Message) -> Command<Message>{
        
        Command::none()
    }
    
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let offset_start = self.active_date.first_day_in_month() as i32;
        let offset_end = - (self.active_date.last_day_in_month() as i32);
        let weeks = self.active_date.days_in_month() / 7;
        let content = column![]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .push(make_container_row(offset_start))
            .push(make_container_row(0))
            .push(make_container_row(0))
            .push(make_container_row(0))
            .push(make_container_row(offset_end));
        content.into()
    }

    pub fn handle_event(&mut self, event: iced_core::Event) -> Command<Message>{
        use iced_core::Event::*;
        match event {
            Mouse(e) => {
                if let iced::mouse::Event::WheelScrolled { delta} = e {
                    if let ScrollDelta::Lines { x, y } = delta {
                        if y > 0.0 {
                            self.active_date.add_months(-1);
                        } else {
                            self.active_date.add_months(1);
                        }
                        dbg!(self.active_date);
                    }
                }
            }
            _ => {}
        }
        Command::none()
    }   
}

pub fn make_container<'a>(appointment: Appointment) -> Element<'a, Message> {
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

pub fn make_container_row<'a>(offset: i32) -> Element<'a, Message> {
    let mut content = row![]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);
    for i in 0..7 {
        if offset > 0 && i < offset {
            content = content.push(Container::new("").width(Length::Fill));
        } else if offset > 0 && i >= offset {
            content = content.push(make_container(Appointment::default()));
        } else if offset == 0 {
            content = content.push(make_container(Appointment::default()));
        } else if offset < 0 && i > - offset {
            content = content.push(Container::new("").width(Length::Fill));
        } else if offset < 0 && i <= - offset {
            content = content.push(make_container(Appointment::default()));
        }
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
