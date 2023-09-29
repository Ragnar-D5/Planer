use serde::{Deserialize, Serialize};
use chrono::naive::{NaiveDateTime, NaiveDate};
use chrono::{Datelike, Timelike, NaiveTime};


#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct PDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub min: u32,
    pub sec: u32
}

impl Default for PDate {
    fn default() -> Self {
        PDate { year: 1970, month: 1, day: 1, hour: 0, min: 0, sec: 0 }
    }
}

impl PDate {

    pub fn new(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Self {
        Self { year, month, day, hour, min, sec }
    }

    pub fn now() -> Self {
        let date = chrono::offset::Local::now().date_naive();
        let time = chrono::offset::Local::now().time();
        Self { year: date.year_ce().1 as i32, month: date.month(), day: date.day(), hour: time.hour(), min: time.minute(), sec: time.second() } // Some(date.iso_week().week())
        
    }

    pub fn day_string(self) -> String {
        format!("{}.{}", self.day, self.month)
    }

    pub fn fmt(self) -> String {
        format!("{}.{}.{}", self.day, self.month, self.year)
    }

}

pub fn now() -> NaiveDateTime {
    chrono::offset::Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap()
}

pub fn new_date( year: i32, month: u32, day: u32 ) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

pub fn new_time( year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32 ) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(year, month, day).unwrap().and_hms_opt(hour, min, sec).unwrap()
}

pub fn format_dmy( date: NaiveDateTime ) -> String {
    format!("{}.{}.{}", date.day(), date.month(), date.year())
}

pub fn first_day_in_month( date: NaiveDateTime ) -> i32 {
    let nd = NaiveDate::from_ymd_opt( date.year(), date.month(), 1)
        .unwrap();
    nd
        .weekday()
        .num_days_from_monday() as i32
}

pub fn last_day_in_month( date: NaiveDateTime ) -> i32 {
    NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .weekday()
        .num_days_from_monday() as i32
}

pub fn days_in_month( date: NaiveDateTime ) -> i32 {
    NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap())
        .signed_duration_since(NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap())
        .num_days() as i32
}

pub fn day_string( date: &NaiveDateTime ) -> String {
    format!("{}.{}", date.day(), date.month())
}

pub fn naive_date_time_to_p_date( date: NaiveDateTime ) -> PDate {
    PDate { year: date.year(), month: date.month(), day: date.day(), hour: date.hour(), min: date.minute(), sec: date.second() }
}

pub fn p_date_to_naive_date_time( date: PDate ) -> NaiveDateTime {
    NaiveDateTime::new(NaiveDate::from_ymd_opt(date.year, date.month, date.day).unwrap(), NaiveTime::from_hms_opt(date.hour, date.min, date.sec).unwrap())
}

pub fn naive_date_time_as_string( date: NaiveDateTime ) -> String {
    format!("{}.{}.{}", date.day(), date.month(), date.year())
}