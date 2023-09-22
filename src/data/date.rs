use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};
use std::ops::Sub;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Date {
    pub year: i32,
    pub month: Option<u32>,
    pub week: Option<u32>,
    pub day: Option<u32>,
}

impl Default for Date {
    fn default() -> Self {
        Date { year: 1970, month: Some(1), week: Some(1), day: Some(1) }
    }
}

impl Date {

    pub fn new(year: i32, month: Option<u32>, week: Option<u32>, day: Option<u32>) -> Self {
        Self { year: year, month: month, week: week, day: day }
    }

    pub fn now() -> Self {
        let date = chrono::offset::Local::now().date_naive();
        Self { year: date.year_ce().1 as i32, month: Some(date.month()), week: Some(date.iso_week().week()), day: Some(date.day()) }
        
    }

    pub fn first_day_in_month(self) -> u32 {
        let nd = NaiveDate::from_ymd_opt(self.year, self.month.unwrap(), 1);
        nd.unwrap()
        .weekday()
        .num_days_from_monday()
    }

    pub fn last_day_in_month(self) -> u32 {
        NaiveDate::from_ymd_opt(self.year, self.month.unwrap() + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(self.year + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap()
            .weekday()
            .num_days_from_monday()
    }

    pub fn days_in_month(self) -> u32 {
        NaiveDate::from_ymd_opt(self.year, self.month.unwrap() + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(self.year + 1, 1, 1).unwrap())
            .signed_duration_since(NaiveDate::from_ymd_opt(self.year, self.month.unwrap(), 1).unwrap())
            .num_days() as u32
    }

    pub fn add_months(&mut self, months: i32) {
        self.month = Some((self.month.unwrap() as i32 + months) as u32);
    }

}