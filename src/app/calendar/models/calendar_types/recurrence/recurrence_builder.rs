use std::collections::HashSet;

use super::{
    date::Date, recurrence_positions::RecurrencePositions, recurrence_vec::RecurrenceVec,
    weekday::Weekday, Frequency, Recurrence, RecurrenceDay,
};

pub struct RecurrenceBuilder {
    frequency: Frequency,

    interval: u32,

    count: Option<u32>,

    until_date: Option<Date>,

    week_start: Option<Weekday>,

    excluded_dates: Option<HashSet<Date>>,

    recurrences: Option<Vec<RecurrenceDay>>,
    weekdays: Option<Vec<Weekday>>,
    positions: Option<Vec<i32>>,

    hours: Option<Vec<u32>>,
    minutes: Option<Vec<u32>>,
    seconds: Option<Vec<u32>>,
    year_days: Option<Vec<u32>>,
    month_days: Option<Vec<u32>>,
    months: Option<Vec<u32>>,
}

impl RecurrenceBuilder {
    pub fn new(frequency: Frequency) -> Self {
        Self {
            frequency,
            interval: 1,
            count: None,
            until_date: None,
            week_start: None,
            excluded_dates: None,
            recurrences: None,
            weekdays: None,
            positions: None,
            hours: None,
            minutes: None,
            seconds: None,
            year_days: None,
            month_days: None,
            months: None,
        }
    }

    pub fn set_interval(&mut self, interval: u32) -> &mut Self {
        self.interval = interval;
        self
    }

    pub fn set_count(&mut self, count: u32) -> &mut Self {
        self.count = Some(count);
        self
    }

    pub fn set_until_date(&mut self, until_date: Date) -> &mut Self {
        self.until_date = Some(until_date);
        self
    }

    pub fn set_week_start(&mut self, week_start: Weekday) -> &mut Self {
        self.week_start = Some(week_start);
        self
    }

    pub fn set_excluded_dates(&mut self, excluded_dates: HashSet<Date>) -> &mut Self {
        self.excluded_dates = Some(excluded_dates);
        self
    }

    pub fn set_recurrences(&mut self, recurrences: Vec<RecurrenceDay>) -> &mut Self {
        self.recurrences = Some(recurrences);
        self
    }

    pub fn set_weekdays(&mut self, weekdays: Vec<Weekday>) -> &mut Self {
        self.weekdays = Some(weekdays);
        self
    }

    pub fn set_positions(&mut self, positions: Vec<i32>) -> &mut Self {
        self.positions = Some(positions);
        self
    }

    pub fn set_hours(&mut self, hours: Vec<u32>) -> &mut Self {
        self.hours = Some(hours);
        self
    }

    pub fn set_minutes(&mut self, minutes: Vec<u32>) -> &mut Self {
        self.minutes = Some(minutes);
        self
    }

    pub fn set_seconds(&mut self, seconds: Vec<u32>) -> &mut Self {
        self.seconds = Some(seconds);
        self
    }

    pub fn set_year_days(&mut self, year_days: Vec<u32>) -> &mut Self {
        self.year_days = Some(year_days);
        self
    }

    pub fn set_month_days(&mut self, month_days: Vec<u32>) -> &mut Self {
        self.month_days = Some(month_days);
        self
    }

    pub fn set_months(&mut self, months: Vec<u32>) -> &mut Self {
        self.months = Some(months);
        self
    }

    pub fn build(&self) -> Recurrence {
        Recurrence {
            frequency: self.frequency,
            interval: self.interval,
            count: self.count,
            until_date: self.until_date,
            week_start: self.week_start,
            excluded_dates: self.excluded_dates.clone().unwrap_or_default(),
            recurrences: self.recurrences.clone().unwrap_or_default(),
            positions: RecurrencePositions::new(self.positions.clone().unwrap_or_default()),
            weekdays: RecurrenceVec::new(self.weekdays.clone().unwrap_or_default()),
            hours: RecurrenceVec::new(self.hours.clone().unwrap_or_default()),
            minutes: RecurrenceVec::new(self.minutes.clone().unwrap_or_default()),
            seconds: RecurrenceVec::new(self.seconds.clone().unwrap_or_default()),
            year_days: RecurrenceVec::new(self.year_days.clone().unwrap_or_default()),
            month_days: RecurrenceVec::new(self.month_days.clone().unwrap_or_default()),
            months: RecurrenceVec::new(self.months.clone().unwrap_or_default()),
        }
    }
}
