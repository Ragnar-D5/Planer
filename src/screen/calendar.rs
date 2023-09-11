use iced::widget::{row, button::Button};
use iced::{Element, Length};

pub struct Calendar;

#[derive(Debug, Clone)]
pub enum Message {
    TimeIncrement,
    TimeDecrement,
}

impl Calendar {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let button_increment = Button::new("Increment")
            .on_press(Message::TimeIncrement);
        let button_decrement = Button::new("Decrement")
            .on_press(Message::TimeDecrement);

        let content = row![]
            .push(button_increment)
            .push(button_decrement)
            .width(Length::Fill)
            .height(Length::Fill);

        content.into()
    }
}