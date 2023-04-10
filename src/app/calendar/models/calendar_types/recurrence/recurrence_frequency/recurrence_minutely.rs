use crate::app::calendar::models::calendar_types::recurrence::{date::Date, Recurrence};

impl Recurrence {
    pub fn calculate_ocurrences_minutely(&self, start_date: Date, ending_date: Date, count: u32) -> Vec<Date> {
        /* If freq is MINUTELY, we advance every `interval` minutes, but:
                - Only in the months specified in `months`,
                - Only in the days specified in `month_days` and `year_days` and `weekdays`,
                - Only in the hours specified in `hours`,
            Only accept dates if the minutes are specified in `seconds`,
            If seconds are specified, expand dates to include all those seconds, else use start_date seconds
            If positions are specified, within a minute only include those that match the positions in positions.
        */
        let mut ocurrences = Vec::new();
        let mut count = count;
        let mut current_date = start_date;

        let use_positions = !self.seconds.is_empty();
        let months = self.months.get_or_default_months();
        let year_days = self.year_days.get_or_default_year_days();
        let month_days = self.month_days.get_or_default_month_days();
        let weekdays = self.weekdays.get_or_default_weekdays();
        let hours = self.hours.get_or_default_hours();
        let minutes = self.minutes.get_or_default_minutes();
        let seconds = self.seconds.get_or_default(vec![start_date.get_second()]);

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
            }

            if let Some(date) = skip_to_date {
                let minutes_to_add = self.calculate_interval_to_skip_ocurrence(current_date.minutes_to_date(&date));
                current_date = current_date.add_minutes(minutes_to_add);
                continue;
            }

            if !minutes.contains(&current_date.get_minute()) {
                // Minute not valid. Continue
                current_date = current_date.add_minutes(self.interval);
                continue;
            }

            // Expand seconds
            let mut minute_ocurrences = Vec::new();
            for second in seconds.iter() {
                let ocurrence = current_date.set_second(*second, false);
                if let Some(minute_ocurrence) = ocurrence {
                    minute_ocurrences.push(minute_ocurrence);
                }
            }

            let minute_ocurrences_filtered = if use_positions {
                self.positions.apply(minute_ocurrences)
            } else {
                minute_ocurrences
            };

            for ocurrence in minute_ocurrences_filtered.iter() {
                if !self.excluded_dates.contains(ocurrence)
                    && count > 0
                    && ocurrence <= &ending_date
                    && ocurrence >= &start_date
                {
                    ocurrences.push(*ocurrence);
                    count -= 1;
                }
            }

            current_date = current_date.add_minutes(self.interval);
        }

        ocurrences
    }
}
