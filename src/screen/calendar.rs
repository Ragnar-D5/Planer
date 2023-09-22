use iced::widget::Text;
use iced::widget::{row, button::Button, Container, column, container::Appearance};
use iced::{Element, Length, Command};
use iced_core::mouse::ScrollDelta;
use crate::data::{Date, Appointment, read_appointments};

#[derive(Clone)]
pub struct Calendar {
    pub active_date: Date,
}

#[derive(Clone)]
pub struct CalendarWidget {
    active_date: Date,
    appointments: Vec<Appointment>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TimeIncrement,
    TimeDecrement,
}

impl CalendarWidget{

    pub fn new(calendar: Calendar) -> Self {
        CalendarWidget { active_date: Date::now(), appointments: read_appointments() }
    }

    pub fn update(&mut self, message: Message) -> Command<Message>{
        
        Command::none()
    }
    
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let offset_start = self.active_date.first_day_in_month() as i32;
        let offset_end = - (self.active_date.last_day_in_month() as i32);
        dbg!(&offset_end);
        let mut weeks = (self.active_date.days_in_month() as i32 - 7 + offset_start + offset_end) / 7 ;
        if offset_end == 0 {
            weeks -= 1;
        }
        dbg!(&weeks);
        let mut first_date = self.active_date;
        first_date.day = Some(1);
        let mut content = column![]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        for i in 0..(weeks + 2) {
            if i == 0 {
                content = content
                    .push(self.make_container_row(offset_start, first_date));
                first_date.add_days(7 - offset_start)
            } else if i == weeks + 1 {
                content = content
                    .push(self.make_container_row(offset_end, first_date));
            } else {
                content = content
                    .push(self.make_container_row(0, first_date));
                first_date.add_days(7)
            }
        }

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
                    }
                }
            }
            _ => {}
        }
        Command::none()
    }   

    pub fn make_container<'a>(&self, appointment: Option<&Appointment>, date: Date) -> Element<'a, Message> {
        let mut content = column![]
            .push(Text::new(date.day_string()));
        if appointment != None {
            content = content.push(Button::new(iced::widget::text(appointment.unwrap().description())).width(Length::Fill))
        }
        content = content.push(Button::new("+").width(Length::Fill));
        let mut container = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .style(DayContainer::new().move_to_style())
            .into();
    
        container
    }
    
    pub fn make_container_row<'a>(&self, offset: i32, mut first_date: Date) -> Element<'a, Message> {
        let mut content = row![]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);
        for i in 0..7 {
            let app_today = 'found: {
                for appointment in &self.appointments {
                    if appointment.date == first_date {
                        break 'found Some(appointment);
                    }
                }
                None
            };
            
            
            if offset > 0 && i < offset {
                content = content.push(Container::new("").width(Length::Fill));
            } else if offset > 0 && i >= offset {
                content = content.push(self.make_container(app_today, first_date));
                first_date.add_days(1)
            } else if offset == 0 {
                content = content.push(self.make_container(app_today, first_date));
                first_date.add_days(1)
            } else if offset < 0 && i > - offset {
                content = content.push(Container::new("").width(Length::Fill));
            } else if offset < 0 && i <= - offset {
                content = content.push(self.make_container(app_today, first_date));
                first_date.add_days(1)
            }
        }
        content
            .into()
    }
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
