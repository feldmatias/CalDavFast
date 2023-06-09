use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};

use super::{recurrence_vec::RecurrenceVec, weekday::Weekday};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
        let current_month = current_date.month();

        current_date = current_date
            .with_nanosecond(0)?
            .with_second(0)?
            .with_minute(0)?
            .with_hour(0)?
            .with_day(1)?
            .with_month(month)?;

        if month <= current_month {
            current_date = current_date.with_year(current_date.year() + 1)?;
        }

        Some(Self::new(current_date))
    }

    pub fn set_month_day(&self, day: u32) -> Option<Self> {
        let mut current_date = self.date;
        let current_day = current_date.day();

        current_date = current_date
            .with_nanosecond(0)?
            .with_second(0)?
            .with_minute(0)?
            .with_hour(0)?
            .with_day(1)?;

        if day <= current_day {
            let mut next_month = current_date.month() + 1;
            if next_month > 12 {
                next_month = 1;
            }
            current_date = self.set_month(next_month)?.date;
        }

        current_date = current_date.with_day(day)?;

        Some(Self::new(current_date))
    }

    pub fn set_year_day(&self, day: u32) -> Option<Self> {
        let mut current_date = self.date;
        let current_day = current_date.ordinal();

        current_date = current_date
            .with_nanosecond(0)?
            .with_second(0)?
            .with_minute(0)?
            .with_hour(0)?
            .with_ordinal(1)?;

        if day <= current_day {
            current_date = current_date.with_year(current_date.year() + 1)?
        }

        current_date = current_date.with_ordinal(day)?;

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

    pub fn set_hour(&self, hour: u32, maintain_consistency: bool) -> Option<Self> {
        let mut current_date = self.date;

        if maintain_consistency && hour <= current_date.hour() {
            current_date += chrono::Duration::days(1);
        }

        current_date = current_date
            .with_hour(hour)?
            .with_minute(0)?
            .with_second(0)?
            .with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_minute(&self, minute: u32, maintain_consistency: bool) -> Option<Self> {
        let mut current_date = self.date;

        if maintain_consistency && minute <= current_date.minute() {
            current_date += chrono::Duration::hours(1);
        }

        current_date = current_date.with_minute(minute)?.with_second(0)?.with_nanosecond(0)?;

        Some(Self::new(current_date))
    }

    pub fn set_second(&self, second: u32, maintain_consistency: bool) -> Option<Self> {
        let mut current_date = self.date;

        if maintain_consistency && second <= current_date.second() {
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
    pub fn advance_until_next_available_month(&self, available: &RecurrenceVec<u32>) -> Self {
        let mut next_month = available.get_next(&self.get_month());
        let mut new_date = self.set_month(next_month);
        while new_date.is_none() {
            next_month = available.get_next(&next_month);
            new_date = self.set_month(next_month);
        }
        new_date.unwrap()
    }

    pub fn advance_until_next_available_month_day(&self, available: &RecurrenceVec<u32>) -> Self {
        let mut next_day = available.get_next(&self.get_month_day());
        let mut new_date = self.set_month_day(next_day);
        while new_date.is_none() {
            next_day = available.get_next(&next_day);
            new_date = self.set_month_day(next_day);
        }
        new_date.unwrap()
    }

    pub fn advance_until_next_available_year_day(&self, available: &RecurrenceVec<u32>) -> Self {
        let mut next_day = available.get_next(&self.get_year_day());
        let mut new_date = self.set_year_day(next_day);
        while new_date.is_none() {
            next_day = available.get_next(&next_day);
            new_date = self.set_year_day(next_day);
        }
        new_date.unwrap()
    }

    pub fn advance_until_next_available_weekday(&self, available: &RecurrenceVec<Weekday>) -> Self {
        let mut next_weekday = available.get_next(&self.get_weekday());
        let mut new_date = self.set_weekday(next_weekday);
        while new_date.is_none() {
            next_weekday = available.get_next(&next_weekday);
            new_date = self.set_weekday(next_weekday);
        }
        new_date.unwrap()
    }

    pub fn advance_until_next_available_hour(&self, available: &RecurrenceVec<u32>) -> Self {
        let mut next_hour = available.get_next(&self.get_hour());
        let mut new_date = self.set_hour(next_hour, true);
        while new_date.is_none() {
            next_hour = available.get_next(&next_hour);
            new_date = self.set_hour(next_hour, true);
        }
        new_date.unwrap()
    }

    pub fn advance_until_next_available_minute(&self, available: &RecurrenceVec<u32>) -> Self {
        let mut next_minute = available.get_next(&self.get_minute());
        let mut new_date = self.set_minute(next_minute, true);
        while new_date.is_none() {
            next_minute = available.get_next(&next_minute);
            new_date = self.set_minute(next_minute, true);
        }
        new_date.unwrap()
    }

    pub fn advance_until_next_available_second(&self, available: &RecurrenceVec<u32>) -> Self {
        let mut next_second = available.get_next(&self.get_second());
        let mut new_date = self.set_second(next_second, true);
        while new_date.is_none() {
            next_second = available.get_next(&next_second);
            new_date = self.set_second(next_second, true);
        }
        new_date.unwrap()
    }
}

impl Date {
    pub fn add_seconds(&self, seconds: u32) -> Self {
        Self::new(self.date + chrono::Duration::seconds(seconds as i64))
    }

    pub fn add_minutes(&self, minutes: u32) -> Self {
        Self::new(self.date + chrono::Duration::minutes(minutes as i64))
    }

    pub fn add_hours(&self, hours: u32) -> Self {
        Self::new(self.date + chrono::Duration::hours(hours as i64))
    }
}

impl Date {
    pub fn seconds_to_date(&self, other: &Self) -> u32 {
        (other.date - self.date).num_seconds() as u32
    }

    pub fn minutes_to_date(&self, other: &Self) -> u32 {
        (other.date - self.date).num_minutes() as u32
    }

    pub fn hours_to_date(&self, other: &Self) -> u32 {
        (other.date - self.date).num_hours() as u32
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
