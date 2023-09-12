use chrono::{NaiveDate, Datelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub year: i32,
    pub month: Option<u32>,
    pub week: Option<u32>,
    pub day: Option<u32>,
}

impl Date {

    pub fn first_day(self) -> u32 {
        let nd = NaiveDate::from_ymd_opt(self.year, self.month.unwrap(), 1);
        nd.unwrap().weekday().num_days_from_monday()
    }

}