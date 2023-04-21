use crate::app::calendar::models::calendar_types::recurrence::{
    date::Date, recurrence_vec::RecurrenceVec, weekday::Weekday, Recurrence,
};

use super::recurrence_calculator::RecurrenceFrequencyCalculator;

pub struct MinutelyRecurrenceCalculator {
    months: RecurrenceVec<u32>,
    year_days: RecurrenceVec<u32>,
    month_days: RecurrenceVec<u32>,
    weekdays: RecurrenceVec<Weekday>,
    hours: RecurrenceVec<u32>,
    minutes: RecurrenceVec<u32>,
    seconds: RecurrenceVec<u32>,
}

impl MinutelyRecurrenceCalculator {
    pub fn new(recurrence: &Recurrence, start_date: Date) -> Self {
        let months = recurrence.months.get_or_default_months();
        let year_days = recurrence.year_days.get_or_default_year_days();
        let month_days = recurrence.month_days.get_or_default_month_days();
        let weekdays = recurrence.weekdays.get_or_default_weekdays();
        let hours = recurrence.hours.get_or_default_hours();
        let minutes = recurrence.minutes.get_or_default_minutes();
        let seconds = recurrence.seconds.get_or_default(vec![start_date.get_second()]);

        Self {
            months,
            year_days,
            month_days,
            weekdays,
            hours,
            minutes,
            seconds,
        }
    }
}

impl RecurrenceFrequencyCalculator for MinutelyRecurrenceCalculator {
    /* When freq is MINUTELY, we advance every `interval` minutes, but:
                - Only in the months specified in `months`,
                - Only in the days specified in `month_days` and `year_days` and `weekdays`,
                - Only in the hours specified in `hours`,
            Only accept dates if the minutes are specified in `minutes`,
            If seconds are specified, expand dates to include all those seconds, else use start_date seconds
            If positions are specified, within a minute only include those that match the positions in positions.
    */
    fn use_positions(&self, recurrence: &Recurrence) -> bool {
        !recurrence.seconds.is_empty()
    }

    fn get_skip_time(&self, current_date: Date, interval: u32) -> Option<u32> {
        let mut skip_to_date: Option<Date> = None;
        if !self.months.contains(&current_date.get_month()) {
            // Skip to next month
            skip_to_date = Some(current_date.advance_until_next_available_month(&self.months));
        } else if !self.year_days.contains(&current_date.get_year_day()) {
            // Skip to next year day
            skip_to_date = Some(current_date.advance_until_next_available_year_day(&self.year_days));
        } else if !self.month_days.contains(&current_date.get_month_day()) {
            // Skip to next month day
            skip_to_date = Some(current_date.advance_until_next_available_month_day(&self.month_days));
        } else if !self.weekdays.contains(&current_date.get_weekday()) {
            // Skip to next weekday
            skip_to_date = Some(current_date.advance_until_next_available_weekday(&self.weekdays));
        } else if !self.hours.contains(&current_date.get_hour()) {
            // Skip to next hour
            skip_to_date = Some(current_date.advance_until_next_available_hour(&self.hours));
        }

        skip_to_date
            .map(|date| self.calculate_interval_to_skip_ocurrence(current_date.minutes_to_date(&date), interval))
    }

    fn add_time(&self, current_date: Date, time: u32) -> Date {
        current_date.add_minutes(time)
    }

    fn check_date(&self, current_date: Date) -> bool {
        self.minutes.contains(&current_date.get_minute())
    }

    fn expand_date(&self, current_date: Date) -> Vec<Date> {
        // Expand minutes and seconds
        let mut minute_ocurrences = Vec::new();
        for second in self.seconds.iter() {
            let ocurrence = current_date.set_second(*second, false);
            if let Some(minute_ocurrence) = ocurrence {
                minute_ocurrences.push(minute_ocurrence);
            }
        }
        minute_ocurrences
    }
}
