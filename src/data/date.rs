use chrono::{NaiveDate, DateTime, Local, Datelike};

pub struct Date;

impl Date {

    pub fn start_of_month_in_year(year: i32, month: u32) -> u8 {
        let nd = NaiveDate::from_ymd_opt(year, month, 1);
        nd.unwrap().weekday().num_days_from_monday()
    }

}