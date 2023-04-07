use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};

use super::weekday::Weekday;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Date {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
}

impl Date {
    pub fn new(date: DateTime<Utc>) -> Self {
        Self { date }
    }

    pub fn set_month(&self, month: u32) -> Option<Self> {
        let mut current_date = self.date;
        let month = month;

        if month <= current_date.month() {
            current_date = current_date.with_year(current_date.year() + 1)?;
        }

        current_date = current_date
            .with_month(month)?
            .with_day(1)?
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_month_day(&self, day: u32) -> Option<Self> {
        let mut current_date = self.date;

        if day <= current_date.day() {
            let mut next_month = current_date.month() + 1;
            if next_month > 12 {
                next_month = 1;
            }
            current_date = self.set_month(next_month)?.date;
        }

        current_date = current_date
            .with_day(day)?
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_year_day(&self, day: u32) -> Option<Self> {
        let mut current_date = self.date;

        if day <= current_date.ordinal() {
            current_date = current_date.with_year(current_date.year() + 1)?
        }

        current_date = current_date
            .with_ordinal(day)?
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_weekday(&self, weekday: Weekday) -> Option<Self> {
        let mut current_date = self.date;

        let days_diff = weekday.get_days_diff_from(&Weekday::from_chrono(&current_date.weekday()));
        current_date += chrono::Duration::days(days_diff as i64);
        current_date = current_date
            .with_hour(0)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_hour(&self, hour: u32) -> Option<Self> {
        let mut current_date = self.date;

        if hour <= current_date.hour() {
            current_date += chrono::Duration::days(1);
        }

        current_date = current_date
            .with_hour(hour)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_minute(&self, minute: u32) -> Option<Self> {
        let mut current_date = self.date;

        if minute <= current_date.minute() {
            current_date += chrono::Duration::hours(1);
        }

        current_date = current_date
            .with_minute(minute)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_second(&self, second: u32) -> Option<Self> {
        let mut current_date = self.date;

        if second <= current_date.second() {
            current_date += chrono::Duration::minutes(1);
        }

        current_date = current_date.with_second(second)?.with_nanosecond(0)?;

        Some(Self::new(current_date))
    }
}

impl Date {
    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_month_day(&self) -> u32 {
        self.date.day()
    }

    pub fn get_year_day(&self) -> u32 {
        self.date.ordinal()
    }

    pub fn get_weekday(&self) -> Weekday {
        Weekday::from_chrono(&self.date.weekday())
    }

    pub fn get_hour(&self) -> u32 {
        self.date.hour()
    }

    pub fn get_minute(&self) -> u32 {
        self.date.minute()
    }

    pub fn get_second(&self) -> u32 {
        self.date.second()
    }
}

impl Date {
    pub fn add_seconds(&self, seconds: u32) -> Self {
        Self::new(self.date + chrono::Duration::seconds(seconds as i64))
    }
}

impl std::cmp::PartialEq<DateTime<Utc>> for Date {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.date == *other
    }
}

impl std::cmp::PartialOrd<DateTime<Utc>> for Date {
    fn partial_cmp(&self, other: &DateTime<Utc>) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(other)
    }
}

impl std::cmp::PartialEq<Date> for Date {
    fn eq(&self, other: &Date) -> bool {
        self.date == other.date
    }
}

impl std::cmp::PartialOrd<Date> for Date {
    fn partial_cmp(&self, other: &Date) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Clone for Date {
    fn clone(&self) -> Self {
        Self::new(self.date)
    }
}

impl Copy for Date {}
