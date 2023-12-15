use std::fmt::Debug;

use chrono::naive::{NaiveDateTime, Days};
use chrono::naive::NaiveDate;
use chrono::{Datelike, Months};
use iced::widget::{Text, text, button, container, text_input, PickList, Space, self, scrollable, Row};
use iced::widget::{column, button::Button, Container, row, button::Appearance};
use iced::{Element, Length, Command, theme, window};
use iced_core::{Widget, Vector};
use iced_core::keyboard::{KeyCode, Modifiers};
use iced_core::mouse::ScrollDelta;
use iced_core::alignment::Horizontal;

use crate::data::{Appointment, read_appointments, save_appointments, Priority};
use crate::screen::modal_overlay::Modal;
use crate::data::{PDate, date};


#[derive(Clone)]
pub struct Calendar {
    pub active_date: NaiveDateTime,
}

#[derive(Clone, Debug)]
enum Depth {
    Year = 0,
    Month = 1,
    Week = 2,
}

impl Depth {

    pub const fn depth_increase(&self) -> Depth {
        match *self {
            Depth::Year => Depth::Month,
            Depth::Month => Depth::Week,
            Depth::Week => Depth::Week,
        }
    }

    pub const fn depth_decrease(&self) -> Depth {
        match *self {
            Depth::Year => Depth::Year,
            Depth::Month => Depth::Year,
            Depth::Week => Depth::Month,         
        }
    }
}

#[derive(Clone)]
pub struct CalendarWidget {
    active_date: NaiveDateTime,
    depth: Depth,
    appointments: Vec<Appointment>,
    edit_dialog: Option<DialogOption>,
    dialog_appointment: DialogAppointment,
    modifiers: Modifiers,
}

#[derive(Clone)]
pub struct DialogAppointment {
    date: String,
    priority: Priority,
    warning: String,
    tags: String,
    description: String,
}

impl Default for DialogAppointment {
    fn default() -> Self {
        DialogAppointment { date: "".to_string(), priority: Priority::Low, warning: "".to_string(), tags: "".to_string(), description: "".to_string() }
    }
}

impl DialogAppointment {
    fn from_appointment(appointment: Appointment) -> Self {
        let tags = appointment.tags.unwrap().join(", ");
        DialogAppointment { date: appointment.date.fmt(), priority: appointment.priority, warning: appointment.warning.fmt(), tags, description: appointment.description }
    }
}

#[derive(PartialEq, Clone)]
enum DialogOption {
    Edit(Appointment),
    Add(NaiveDateTime)
}

#[derive(Debug, Clone)]
pub enum Message {
    TimeIncrement,
    TimeDecrement,
    AddAppointment(NaiveDateTime),
    EditAppointment(i32),
    DialogPriority(Priority),
    DialogDate(String),
    DialogWarning(String),
    DialogTags(String),
    DialogDescription(String),
    DialogCancel,
    DialogSubmit(Option<Appointment>),
}

impl CalendarWidget{

    pub fn new() -> Self {
        CalendarWidget { 
            active_date: date::now(), 
            depth: Depth::Month,
            appointments: read_appointments(), 
            edit_dialog: None, 
            dialog_appointment: DialogAppointment::default(), 
            modifiers: Modifiers::empty() 
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message>{
        match message {
            Message::TimeIncrement => {
                self. active_date = match self.depth {
                    Depth::Week => self.active_date.checked_add_days(Days::new(7))
                    .unwrap_or(self.active_date),
                    Depth::Month => self.active_date.checked_add_months(Months::new(1))
                        .unwrap_or(self.active_date),
                    Depth::Year => self.active_date.checked_add_months(Months::new(12))
                        .unwrap_or(self.active_date)
                };
                Command::none()
            }
            Message::TimeDecrement => {
                self. active_date = match self.depth {
                    Depth::Week => self.active_date.checked_sub_days(Days::new(7))
                    .unwrap_or(self.active_date),
                    Depth::Month => self.active_date.checked_sub_months(Months::new(1))
                        .unwrap_or(self.active_date),
                    Depth::Year => self.active_date.checked_sub_months(Months::new(12))
                        .unwrap_or(self.active_date)
                };
                Command::none()
            }
            Message::AddAppointment(date) => {
                self.edit_dialog = Some(DialogOption::Add(date));
                self.dialog_appointment = DialogAppointment::default();
                self.dialog_appointment.date = date::naive_date_time_as_string(date);
                Command::none()
            }
            Message::EditAppointment(id) => {
                for app in &self.appointments {
                    if id == app.id {
                        let appointment = app.clone();
                        self.edit_dialog = Some(DialogOption::Edit(appointment.clone()));
                        self.dialog_appointment = DialogAppointment::from_appointment(appointment);
                        break;
                    }
                }
                Command::none()
            }
            Message::DialogDate(string) => {
                self.dialog_appointment.date = string.clone();
                Command::none()
            }
            Message::DialogPriority(priority) => {
                self.dialog_appointment.priority = priority;
                widget::focus_next()
            }
            Message::DialogWarning(string) => {
                self.dialog_appointment.warning = string.clone();
                let date = valid_date(string).ok();
                if date != None {
                    widget::focus_next()
                } else {
                    Command::none()
                }
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
            Message::DialogSubmit(appointment) => {
                if valid_date(self.dialog_appointment.date.clone()).is_ok() &&
                    valid_date(self.dialog_appointment.warning.clone()).is_ok() && 
                    valid_tags(self.dialog_appointment.tags.clone()).is_ok() {
                        if appointment != None {
                            let index = self.appointments.iter().position(|x| *x == appointment.clone().unwrap()).unwrap();
                            self.appointments.remove(index);
                        }
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
                        save_appointments(self.appointments.clone());
                        self.edit_dialog = None;
                    }   
                Command::none()
            }
            _ => Command::none()
        }
    }
    
    pub fn view<'a>(&self) -> Element<'a, Message> {
        let content = match self.depth {
            Depth::Year => {
                self.view_year(self.active_date)
            },
            Depth::Month => {
                self.view_month(self.active_date)
            }
            Depth::Week => {
                self.view_week(self.active_date)
            }
        };
        if let Some(DialogOption::Edit(appointment)) = &self.edit_dialog {
            let modal = container(
                column![
                    column![
                        text("Date").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.date.as_str())
                            .on_input(Message::DialogDate)
                    ],
                    column![
                        text("Warning").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.warning.as_str())
                            .on_input(Message::DialogWarning)
                    ],
                    column![
                        text("Tags").size(12),
                        text_input("tag_1, tag_2", self.dialog_appointment.tags.as_str())
                            .on_input(Message::DialogTags)
                    ],
                    column![
                        text("Description").size(12),
                        text_input("", self.dialog_appointment.description.as_str())
                            .on_input(Message::DialogDescription)
                    ],
                    column![
                        text("Priority").size(12),
                        PickList::new(Priority::ALL, Some(self.dialog_appointment.priority), Message::DialogPriority)
                    ],
                    row![
                        button("Cancel")
                            .on_press(Message::DialogCancel),
                        Space::new(Length::Fill, Length::Shrink),
                        button("Submit")
                            .on_press(Message::DialogSubmit(Some(appointment.clone())))
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
        } else if let Some(DialogOption::Add(_date)) = self.edit_dialog {
            let modal = container(
                column![
                    column![
                        text("Date").size(12),
                        text_input("dd.mm.yyyy", self.dialog_appointment.date.as_str())
                            .on_input(Message::DialogDate)
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
                    column![
                        text("Priority").size(12),
                        PickList::new(Priority::ALL, Some(self.dialog_appointment.priority), Message::DialogPriority)
                    ],
                    row![
                        button("Cancel")
                            .on_press(Message::DialogCancel),
                        Space::new(Length::Fill, Length::Shrink),
                        button("Submit")
                            .on_press(Message::DialogSubmit(None))
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

    fn view_week<'a>(&self, active_date: NaiveDateTime) -> Element<'a, Message> {
        let mut content = row![].spacing(10).width(Length::Fill).height(Length::Fill);
        for i in 0..7 {
            let mut column = column![];
            column = match i {
                0 => column.push(text("Monday")),
                1 => column.push(text("Tuesday")),
                2 => column.push(text("Wednesday")),
                3 => column.push(text("Thursday")),
                4 => column.push(text("Friday")),
                5 => column.push(text("Saturday")),
                6 => column.push(text("Sunday")),
                _ => column,
            };
            column = column.push(
                container(scrollable(button("hello")))
            );
            content = content.push(column);
        }
        content.into()
    }    

    fn view_year<'a>(&self, mut active_date: NaiveDateTime) -> Element<'a, Message> {
        active_date = NaiveDate::from_ymd_opt(active_date.year(), 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let mut content = column![].width(Length::Fill).height(Length::Fill);
        for _i in 0..2 {
            let res = self.year_month_row(active_date);
            content = content.push(res.0);
            active_date = res.1;
        }
        content.into()
    }

    fn year_month_row<'a>(&self, mut active_date: NaiveDateTime) -> (Element<'a, Message>, NaiveDateTime) {
        let mut row = row![].spacing(10).width(Length::Fill).height(Length::Fill);
        for _i in 0..6 {
            let res = self.year_month(active_date);
            row = row.push(res.0);
            active_date = res.1;
        }
        (row.into(), active_date)
    }

    fn year_month<'a>(&self, mut active_date: NaiveDateTime) -> (Element<'a, Message>, NaiveDateTime) {
        let mut content =  column![].spacing(5).width(Length::Fill).height(Length::Fill);
        content = match active_date.month() {
            1 => content.push(text("January").horizontal_alignment(Horizontal::Center)),
            2 => content.push(text("February").horizontal_alignment(Horizontal::Center)),
            3 => content.push(text("March").horizontal_alignment(Horizontal::Center)),
            4 => content.push(text("April").horizontal_alignment(Horizontal::Center)),
            5 => content.push(text("May").horizontal_alignment(Horizontal::Center)),
            6 => content.push(text("June").horizontal_alignment(Horizontal::Center)),
            7 => content.push(text("July").horizontal_alignment(Horizontal::Center)),
            8 => content.push(text("August").horizontal_alignment(Horizontal::Center)),
            9 => content.push(text("September").horizontal_alignment(Horizontal::Center)),
            10 => content.push(text("October").horizontal_alignment(Horizontal::Center)),
            11 => content.push(text("November").horizontal_alignment(Horizontal::Center)),
            12 => content.push(text("December").horizontal_alignment(Horizontal::Center)),
            _ => content,
        };
        loop {
            let mut row = row![].spacing(5).width(Length::Fill).height(Length::Fill);
            for i in 0..7 {
                if active_date.day() == 1 && active_date.weekday().num_days_from_monday() != i {
                    row = row.push(container(text("")).width(Length::Fill).height(Length::Fill));
                } else if active_date.checked_add_days(Days::new(1)).unwrap().day() == 1 {
                    row = row.push(container(text("")).width(Length::Fill).height(Length::Fill));
                } else {
                    let _appointments = self.find_appointments_from_date(active_date);
                    row = row.push(button(text("")).style(DayContainer::new().move_to_style()).width(Length::Fill).height(Length::Fill));
                    active_date = active_date.checked_add_days(Days::new(1)).unwrap();
                }

            }
            content = content.push(row);
            if active_date.checked_add_days(Days::new(1)).unwrap().day() == 1 {
                break
            }
        }
        (content.into(), active_date.checked_add_days(Days::new(1)).unwrap())
    }

    fn find_appointments_from_date(&self, active_date: NaiveDateTime) -> Vec<&Appointment> {
        self.appointments.iter().filter(|x| x.date == date::naive_date_time_to_p_date(active_date)).collect()
        
    }

    fn view_month<'a>(&self, mut active_date: NaiveDateTime) -> Element<'a, Message> {
        active_date = NaiveDate::from_ymd_opt(active_date.year(), active_date.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let mut column = column![].spacing(5);
        let month = active_date.month();
        loop {
            let mut row = row![].spacing(5).width(Length::Fill).height(Length::Fill);
            for i in 0..7 {
                dbg!(self.active_date);
                if active_date.day() == 1 && active_date.weekday().num_days_from_monday() != i {
                    row = row.push(container(text("")).width(Length::Fill).height(Length::Fill));
                    println!("not a day");
                } else if month != active_date.month(){ 
                        row = row.push(container(text("")).width(Length::Fill).height(Length::Fill));
                } else {
                        let _appointments = self.find_appointments_from_date(active_date);
                        row = row.push(self.make_container(active_date));
                        active_date = active_date.checked_add_days(Days::new(1)).unwrap();
                }
            }
            column = column.push(row);
        
            if active_date.day() == 1 {
                break
            }
        }
        column.into()
    }
    

    fn make_container<'a>(&self, active_date: NaiveDateTime) -> Element<'a, Message> {
        let appointments = self.find_appointments_from_date(active_date);
        let mut content = column![]
            .push(Text::new(date::naive_date_time_as_string(active_date)));
        for appointment in appointments {
            content = content.push(Button::new(iced::widget::text(appointment.description())).width(Length::Fill)
                .on_press(Message::EditAppointment(appointment.id)))
        }
        // content = content.push(Button::new("+")
        //     .width(Length::Fill)
        //     .on_press(Message::AddAppointment(active_date))
        // );
        let container = Button::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .style(DayContainer::new().move_to_style())
            .on_press(Message::AddAppointment(active_date))
            .into();
    
        container
    }

    pub fn handle_event(&mut self, event: iced_core::Event) -> Command<Message>{
        use iced_core::Event::*;
        match event {
            Mouse(e) => {
                if let iced::mouse::Event::WheelScrolled { delta} = e {
                    if let ScrollDelta::Lines { x: _, y } = delta {
                        if self.modifiers.control() {
                            if y > 0.0 {
                                self.depth = self.depth.depth_decrease();
                            } else {
                                self.depth = self.depth.depth_increase();
                            }
                        } else if y > 0.0 {
                            let _ = self.update(Message::TimeDecrement);
                        } else {
                            let _ = self.update(Message::TimeIncrement);
                        }
                    }
                }
            }
            Window(e) => {                                   
                if let iced::window::Event::CloseRequested = e {    
                    save_appointments(self.appointments.clone());   
                    return window::close()                          
                }
            }
            Keyboard(e) => {
                match e {
                    iced_core::keyboard::Event::KeyPressed { 
                        key_code: KeyCode::Tab,
                        modifiers: _ 
                    } => {
                        return widget::focus_next()
                    }
                    iced_core::keyboard::Event::KeyPressed { 
                        key_code: KeyCode::Escape,
                        modifiers: _ 
                    } => {
                        self.edit_dialog = None;
                    }
                    iced_core::keyboard::Event::KeyPressed { 
                        key_code: KeyCode::Down, 
                        modifiers: _,
                    } => {
                        return self.update(Message::TimeIncrement)
                    }
                    iced_core::keyboard::Event::KeyPressed { 
                        key_code: KeyCode::Up, 
                        modifiers: _,
                    } => {
                        return self.update(Message::TimeDecrement)
                    }
                    iced_core::keyboard::Event::KeyPressed { 
                        key_code: KeyCode::Left, 
                        modifiers: _,
                    } => {
                        self.depth = self.depth.depth_decrease();
                    }
                    iced_core::keyboard::Event::KeyPressed { 
                        key_code: KeyCode::Right, 
                        modifiers: _,
                    } => {
                        self.depth = self.depth.depth_increase();
                    }
                    iced_core::keyboard::Event::ModifiersChanged( modifiers) => {
                        self.modifiers = modifiers
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Command::none()
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

    pub fn move_to_style(self) -> iced::theme::Button {
        self.into()
    }
}

impl std::convert::From<DayContainer> for iced::theme::Button {
    fn from(value: DayContainer) -> Self {
        iced::theme::Button::Custom(Box::new(value))
    }
}

impl iced::widget::button::StyleSheet for DayContainer {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance { 
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: Some(style.palette().text).unwrap(),
            background: Some(iced::Color::TRANSPARENT.into()), 
            border_radius: 6.0.into(), 
            border_width: 2.0.into(), 
            border_color: iced::Color {a: 0.5, ..style.palette().text} 
        }
    }
}

fn valid_date(string: String) -> Result<PDate, String> {
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
        return Ok(PDate::new(year.unwrap(), month.unwrap(), day.unwrap(), 0, 0, 0))
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
