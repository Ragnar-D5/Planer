use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn first_day(self) -> u32 {
        let nd = NaiveDate::from_ymd_opt(self.year, self.month.unwrap(), 1);
        nd.unwrap().weekday().num_days_from_monday()
    }

    pub fn as_str(self) -> String {
        if self.day != None {
            self.day.unwrap().to_string()
        } else {
            "invalid date".to_string()
        }
    } 
}