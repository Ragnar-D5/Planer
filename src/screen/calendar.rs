use std::fmt::Debug;

use chrono::NaiveDate;
use iced::widget::{Text, text, button, container, text_input, PickList, Space};
use iced::widget::{row, button::Button, Container, column, container::Appearance};
use iced::{Element, Length, Command, theme, window};
use iced_core::mouse::ScrollDelta;

use crate::data::{Date, Appointment, read_appointments, save_appointments, Priority};
use crate::screen::modal_overlay::Modal;


#[derive(Clone)]
pub struct Calendar {
    pub active_date: Date,
}

#[derive(Clone)]
pub struct CalendarWidget {
    active_date: Date,
    appointments: Vec<Appointment>,
    edit_dialog: Option<DialogOption>,
    dialog_appointment: DialogAppointment,
}

#[derive(Clone)]
pub struct DialogAppointment {
    date: String,
    priority: Priority,
    warning: String,
    tags: String,
    description: String
}

impl Default for DialogAppointment {
    fn default() -> Self {
        DialogAppointment { date: "".to_string(), priority: Priority::Low, warning: "".to_string(), tags: "".to_string(), description: "".to_string() }
    }
}

#[derive(PartialEq, Clone)]
enum DialogOption {
    Edit(i32),
    Add(Date)
}

#[derive(Debug, Clone)]
pub enum Message {
    TimeIncrement,
    TimeDecrement,
    AddAppointment(Date),
    EditAppointment(i32),
    Dialog(DialogMessage),
    DialogPriority(Priority),
    DialogDate(String),
    DialogWarning(String),
    DialogTags(String),
    DialogDescription(String),
    DialogCancel,
    DialogSubmit(bool,usize),
}

#[derive(Debug, Clone)]
pub enum DialogMessage {
    Date(Date),
    Warning(Date),
    Tag(String),
    Description(String),
    Cancel,
    Submit
}

impl CalendarWidget{

    pub fn new() -> Self {
        CalendarWidget { active_date: Date::now(), appointments: read_appointments(), edit_dialog: None, dialog_appointment: DialogAppointment::default() }
    }

    pub fn update(&mut self, message: Message) -> Command<Message>{
        match message {
            Message::AddAppointment(date) => {
                self.edit_dialog = Some(DialogOption::Add(date));
                self.dialog_appointment = DialogAppointment::default();
                self.dialog_appointment.date = date.fmt();
                Command::none()
            }
            Message::EditAppointment(id) => {
                self.edit_dialog = Some(DialogOption::Edit(id));
                let mut appointment = Appointment::default();
                let mut index = 0;
                for app in &self.appointments {
                    if id == app.id {
                        appointment = app.clone();
                        break;
                    }
                    index += 1;
                };
                let tags_string = match appointment.tags {
                    Some(vec_str) => vec_str.join(", "),
                    None => "".to_string(),
                };
                self.dialog_appointment = 
                    DialogAppointment { 
                        date: appointment.date.fmt().to_string(), 
                        priority: appointment.priority, 
                        warning: appointment.warning.fmt().to_string(), 
                        tags: tags_string, 
                        description: appointment.description.to_string() 
                    };
                Command::none()
            }
            Message::DialogDate(string) => {
                self.dialog_appointment.date = string.clone();
                let _date = valid_date(string).ok();
                // if date != None {
                //     self.dialog_appointment.date = date.unwrap();
                //     widget::focus_next()
                // } else {
                //     Command::none()
                // }
                Command::none()
            }
            Message::DialogPriority(priority) => {
                self.dialog_appointment.priority = priority;
                Command::none()
            }
            Message::DialogWarning(string) => {
                self.dialog_appointment.warning = string.clone();
                let _date = valid_date(string).ok();
                Command::none()
            }
            Message::DialogTags(string) => {
                self.dialog_appointment.tags = string.clone();
                let _tags = valid_tags(string).ok();
                Command::none()
            }
            Message::DialogDescription(string) => {
                self.dialog_appointment.description = string.clone();
                Command::none()
            }
            Message::DialogCancel => {
                self.edit_dialog = None;
                Command::none()
            }
            Message::DialogSubmit(edit,index) => {
                if valid_date(self.dialog_appointment.date.clone()).is_ok() &&
                    valid_date(self.dialog_appointment.warning.clone()).is_ok() && 
                    valid_tags(self.dialog_appointment.tags.clone()).is_ok() {
                        println!("{},{},{}",edit,index,self.appointments.len());
                        if edit{ //not very elegant but should work
                            self.appointments[index] =
                                Appointment {
                                    id: self.appointments[index].id,
                                    date: valid_date(self.dialog_appointment.date.clone()).unwrap(),
                                    priority: self.dialog_appointment.priority,
                                    warning: valid_date(self.dialog_appointment.warning.clone()).unwrap(),
                                    tags: Some(valid_tags(self.dialog_appointment.tags.clone()).unwrap()),
                                    description: self.dialog_appointment.description.clone()
                                }
                        }
                        else{
                            self.appointments.push(
                                Appointment {
                                    id: new_id(self.appointments.clone()),
                                    date: valid_date(self.dialog_appointment.date.clone()).unwrap(),
                                    priority: self.dialog_appointment.priority,
                                    warning: valid_date(self.dialog_appointment.warning.clone()).unwrap(),
                                    tags: Some(valid_tags(self.dialog_appointment.tags.clone()).unwrap()),
                                    description: self.dialog_appointment.description.clone()
                                }
                            );
                        }
                        println!("{}",self.appointments.len());
                        save_appointments(self.appointments.clone());
                        self.edit_dialog = None;
                    }
                // function for saving this stuff
                Command::none()
            }
            _ => Command::none()
        }
    }
    
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let offset_start = self.active_date.first_day_in_month() as i32;
        let offset_end = - (self.active_date.last_day_in_month() as i32);
        let mut weeks = (self.active_date.days_in_month() as i32 - 7 + offset_start + offset_end) / 7 ;
        if offset_end == 0 {
            weeks -= 1;
        }
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

        if let Some(DialogOption::Edit(id)) = self.edit_dialog {
            let mut appointment = Appointment::default();
            let mut index = 0;
            for app in &self.appointments {
                if id == app.id {
                    appointment = app.clone();
                    break;
                }
                index += 1;
            };
            let modal = container(
                column![
                    column![
                        text("Date").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.date.as_str())
                            .on_input(Message::DialogDate)
                    ],
                    column![
                        text("Priority").size(12),
                        PickList::new(Priority::ALL, Some(self.dialog_appointment.priority), Message::DialogPriority)
                    ],
                    column![
                        text("Warning").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.warning.as_str())
                            .on_input(Message::DialogWarning)
                    ],
                    column![
                        text("Tags").size(12),
                        text_input("tag_1, tag_2",self.dialog_appointment.tags.as_str())
                            .on_input(Message::DialogTags)
                    ],
                    column![
                        text("Description").size(12),
                        text_input("", self.dialog_appointment.description.as_str())
                            .on_input(Message::DialogDescription)
                    ],
                    row![
                        button("Cancel")
                            .on_press(Message::DialogCancel),
                        Space::new(Length::Fill, Length::Shrink),
                        button("Submit")
                            .on_press(Message::DialogSubmit(true,index))
                        ]
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(theme::Container::Box);
            
            Modal::new(content, modal).into()
        } else if let Some(DialogOption::Add(_date)) = self.edit_dialog {
            let modal = container(
                column![
                    column![
                        text("Date").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.date.as_str())
                            .on_input(Message::DialogDate)
                    ],
                    column![
                        text("Priority").size(12),
                        PickList::new(Priority::ALL, Some(self.dialog_appointment.priority), Message::DialogPriority)
                    ],
                    column![
                        text("Warning").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.warning.as_str())
                            .on_input(Message::DialogWarning)
                    ],
                    column![
                        text("Tags").size(12),
                        text_input("tag_1, tag_2",self.dialog_appointment.tags.as_str())
                            .on_input(Message::DialogTags)
                    ],
                    column![
                        text("Description").size(12),
                        text_input("", self.dialog_appointment.description.as_str())
                            .on_input(Message::DialogDescription)
                    ],
                    row![
                        button("Cancel")
                            .on_press(Message::DialogCancel),
                        Space::new(Length::Fill, Length::Shrink),
                        button("Submit")
                            .on_press(Message::DialogSubmit(false,0))
                        ]
                ]
                .spacing(20),
            )
            .width(300)
            .padding(10)
            .style(theme::Container::Box);
            
            Modal::new(content, modal)
                .on_blur(Message::DialogCancel)
                .into()
        } else {
            return content.into()
        }
    }

    pub fn handle_event(&mut self, event: iced_core::Event) -> Command<Message>{
        use iced_core::Event::*;
        match event {
            Mouse(e) => {
                if let iced::mouse::Event::WheelScrolled { delta} = e {
                    if let ScrollDelta::Lines { x: _, y } = delta {
                        if y > 0.0 {
                            self.active_date.add_months(-1);
                        } else {
                            self.active_date.add_months(1);
                        }
                    }
                }
            }
            Window(e) => {
                if let iced::window::Event::CloseRequested = e {
                    dbg!(&self.appointments);
                    save_appointments(self.appointments.clone());
                    return window::close()
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
            content = content.push(Button::new(iced::widget::text(appointment.unwrap().description())).width(Length::Fill)
                .on_press(Message::EditAppointment(appointment.unwrap().id)))
        }
        content = content.push(Button::new("+")
            .width(Length::Fill)
            .on_press(Message::AddAppointment(date))
        );
        let container = Container::new(content)
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

fn valid_date(string: String) -> Result<Date, String> {
    let time: Vec<&str> = string.split(".").collect();
    if time.len() != 3 {
        return Err(string)
    }
    let day = time[0].parse::<u32>();
    let month = time[1].parse::<u32>();
    let year = time[2].parse::<i32>();
    if day.is_err() | month.is_err() | year.is_err() {
        return Err(string)
    } else if time[2].as_bytes().len() != 4 {
        return Err(string)
    } else if NaiveDate::from_ymd_opt(year.clone().unwrap(), month.clone().unwrap(), day.clone().unwrap()).is_none() {
        return Err(string)
    } else {
        return Ok(Date::new(year.unwrap(), Some(month.unwrap()), None, Some(day.unwrap())))
    }

}

fn valid_tags(string: String) -> Result<Vec<String>, String> {
    let tags: Vec<&str> = string.split(",").collect();
    let mut tags_string: Vec<String> = vec![];
    for mut tag in tags {
        tag = tag.trim();
        tags_string.push(tag.to_string())
    }
    return Ok(tags_string)
}

fn new_id(appointments: Vec<Appointment>) -> i32 {
    let mut ids: Vec<i32> = vec![];
    for appointment in appointments {
        ids.push(appointment.id);
    }
    ids.sort();
    for i in 0..ids.len() {
        if i as i32 != ids[i] {
            return i as i32
        }
    }
    return ids.len() as i32
}