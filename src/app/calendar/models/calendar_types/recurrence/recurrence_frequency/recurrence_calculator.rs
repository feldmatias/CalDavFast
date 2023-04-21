use crate::app::calendar::models::calendar_types::recurrence::{date::Date, Recurrence};

pub trait RecurrenceFrequencyCalculator {
    fn use_positions(&self, recurrence: &Recurrence) -> bool;

    fn get_skip_time(&self, current_date: Date, interval: u32) -> Option<u32>;

    fn add_time(&self, current_date: Date, time: u32) -> Date;

    fn check_date(&self, current_date: Date) -> bool;

    fn expand_date(&self, current_date: Date) -> Vec<Date>;

    fn calculate_interval_to_skip_ocurrence(&self, time: u32, interval: u32) -> u32 {
        /* Return time if it is multiple of interval, or the next multiple */
        let modulo = time % interval;
        let result = if modulo == 0 { time } else { time + interval - modulo };

        if result > 0 {
            result
        } else {
            1
        } // Rounding problems
    }
}

pub struct RecurrenceCalculator<'a> {
    frequency_calculator: Box<dyn RecurrenceFrequencyCalculator>,
    recurrence: &'a Recurrence,
}

impl<'a> RecurrenceCalculator<'a> {
    pub fn new(recurrence: &'a Recurrence, frequency_calculator: Box<dyn RecurrenceFrequencyCalculator>) -> Self {
        Self {
            frequency_calculator,
            recurrence,
        }
    }

    pub fn calculate(&self, start_date: Date, ending_date: Date, count: u32) -> Vec<Date> {
        let mut ocurrences = Vec::new();
        let mut count = count;
        let mut current_date = start_date;

        let use_positions = self.frequency_calculator.use_positions(self.recurrence);

        loop {
            if count == 0 {
                break;
            }
            if current_date > ending_date {
                break;
            }

            let skip_time = self
                .frequency_calculator
                .get_skip_time(current_date, self.recurrence.interval);
            if let Some(time_to_skip) = skip_time {
                current_date = self.frequency_calculator.add_time(current_date, time_to_skip);
                continue;
            }

            if !self.frequency_calculator.check_date(current_date) {
                // Date not valid. Continue
                current_date = self
                    .frequency_calculator
                    .add_time(current_date, self.recurrence.interval);
                continue;
            }

            // Expand date
            let expanded_ocurrences = self.frequency_calculator.expand_date(current_date);

            let expanded_ocurrences_filtered = if use_positions {
                self.recurrence.positions.apply(expanded_ocurrences)
            } else {
                expanded_ocurrences
            };

            for ocurrence in expanded_ocurrences_filtered.iter() {
                if !self.recurrence.excluded_dates.contains(ocurrence)
                    && count > 0
                    && ocurrence <= &ending_date
                    && ocurrence >= &start_date
                {
                    ocurrences.push(*ocurrence);
                    count -= 1;
                }
            }

            current_date = self
                .frequency_calculator
                .add_time(current_date, self.recurrence.interval);
        }

        ocurrences
    }
}
