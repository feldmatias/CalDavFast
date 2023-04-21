use crate::app::calendar::models::calendar_types::recurrence::{date::Date, Recurrence};

impl Recurrence {
    pub fn calculate_ocurrences_secondly(&self, start_date: Date, ending_date: Date, count: u32) -> Vec<Date> {
        /* When freq is SECONDLY, we advance every `interval` seconds, but:
                - Only in the months specified in `months`,
                - Only in the days specified in `month_days` and `year_days` and `weekdays`,
                - Only in the hours specified in `hours`,
                - Only in the minutes specified in `minutes`,
            Only accept dates if the seconds are specified in `seconds`,
        */
        let mut ocurrences = Vec::new();
        let mut count = count;
        let mut current_date = start_date;

        let months = self.months.get_or_default_months();
        let year_days = self.year_days.get_or_default_year_days();
        let month_days = self.month_days.get_or_default_month_days();
        let weekdays = self.weekdays.get_or_default_weekdays();
        let hours = self.hours.get_or_default_hours();
        let minutes = self.minutes.get_or_default_minutes();
        let seconds = self.seconds.get_or_default_seconds();

        loop {
            if count == 0 {
                break;
            }
            if current_date > ending_date {
                break;
            }

            let mut skip_to_date: Option<Date> = None;
            if !months.contains(&current_date.get_month()) {
                // Skip to next month
                skip_to_date = Some(current_date.advance_until_next_available_month(&months));
            } else if !year_days.contains(&current_date.get_year_day()) {
                // Skip to next year day
                skip_to_date = Some(current_date.advance_until_next_available_year_day(&year_days));
            } else if !month_days.contains(&current_date.get_month_day()) {
                // Skip to next month day
                skip_to_date = Some(current_date.advance_until_next_available_month_day(&month_days));
            } else if !weekdays.contains(&current_date.get_weekday()) {
                // Skip to next weekday
                skip_to_date = Some(current_date.advance_until_next_available_weekday(&weekdays));
            } else if !hours.contains(&current_date.get_hour()) {
                // Skip to next hour
                skip_to_date = Some(current_date.advance_until_next_available_hour(&hours));
            } else if !minutes.contains(&current_date.get_minute()) {
                // Skip to next minute
                skip_to_date = Some(current_date.advance_until_next_available_minute(&minutes));
            }

            if let Some(date) = skip_to_date {
                let seconds_to_add = self.calculate_interval_to_skip_ocurrence(current_date.seconds_to_date(&date));
                current_date = current_date.add_seconds(seconds_to_add);
                continue;
            }

            if seconds.contains(&current_date.get_second()) {
                count -= 1;
                if !self.excluded_dates.contains(&current_date) {
                    ocurrences.push(current_date);
                }
            }

            current_date = current_date.add_seconds(self.interval);
        }
        ocurrences
    }
}
