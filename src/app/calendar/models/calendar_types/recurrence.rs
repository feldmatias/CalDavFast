/*
The RRULE attribute is used to specify a recurrence rule for a recurring event in iCalendar.
The value of the RRULE attribute is a string that consists of a set of name-value pairs, where the name indicates a parameter of the recurrence rule,
and the value provides the specific value for that parameter.

Here are the possible parameters that can be used in an RRULE, along with their possible values:

FREQ: specifies the frequency of the event's recurrence. Valid values are SECONDLY, MINUTELY, HOURLY, DAILY, WEEKLY, MONTHLY, and YEARLY.

INTERVAL: specifies the number of intervals at which the event recurs. The default value is 1.

COUNT: specifies the number of times the event should recur. If this parameter is not specified, the event will recur indefinitely.

UNTIL: specifies the date and time at which the event should stop recurring. This value should be a date-time value in UTC format.

BYSECOND: specifies a list of seconds within a minute when the event should occur.

BYMINUTE: specifies a list of minutes within an hour when the event should occur.

BYHOUR: specifies a list of hours within a day when the event should occur.

BYDAY: specifies a list of days of the week when the event should occur. Each day can be preceded by a positive or negative integer
to indicate its position within the month or year.

BYMONTHDAY: specifies a list of days of the month when the event should occur.

BYYEARDAY: specifies a list of days of the year when the event should occur.

BYMONTH: specifies a list of months of the year when the event should occur.

BYWEEKNO: specifies a list of ordinals specifying weeks of the year when the event should occur.

BYxxx rule parts modify the recurrence in some manner.  BYxxx rule
      parts for a period of time that is the same or greater than the
      frequency generally reduce or limit the number of occurrences of
      the recurrence generated.  For example, "FREQ=DAILY;BYMONTH=1"
      reduces the number of recurrence instances from all days (if
      BYMONTH rule part is not present) to all days in January.  BYxxx
      rule parts for a period of time less than the frequency generally
      increase or expand the number of occurrences of the recurrence.
      For example, "FREQ=YEARLY;BYMONTH=1,2" increases the number of
      days within the yearly recurrence set from 1 (if BYMONTH rule part
      is not present) to 2.

If multiple BYxxx rule parts are specified, then after evaluating
      the specified FREQ and INTERVAL rule parts, the BYxxx rule parts
      are applied to the current set of evaluated occurrences in the
      following order: BYMONTH, BYWEEKNO, BYYEARDAY, BYMONTHDAY, BYDAY,
      BYHOUR, BYMINUTE, BYSECOND and BYSETPOS; then COUNT and UNTIL are
      evaluated.

+----------+--------+--------+-------+-------+------+-------+------+
   |          |SECONDLY|MINUTELY|HOURLY |DAILY  |WEEKLY|MONTHLY|YEARLY|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYMONTH   |Limit   |Limit   |Limit  |Limit  |Limit |Limit  |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYWEEKNO  |N/A     |N/A     |N/A    |N/A    |N/A   |N/A    |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYYEARDAY |Limit   |Limit   |Limit  |N/A    |N/A   |N/A    |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYMONTHDAY|Limit   |Limit   |Limit  |Limit  |N/A   |Expand |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYDAY     |Limit   |Limit   |Limit  |Limit  |Expand|Note 1 |Note 2|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYHOUR    |Limit   |Limit   |Limit  |Expand |Expand|Expand |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYMINUTE  |Limit   |Limit   |Expand |Expand |Expand|Expand |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYSECOND  |Limit   |Expand  |Expand |Expand |Expand|Expand |Expand|
   +----------+--------+--------+-------+-------+------+-------+------+
   |BYSETPOS  |Limit   |Limit   |Limit  |Limit  |Limit |Limit  |Limit |
   +----------+--------+--------+-------+-------+------+-------+------+

      Note 1:  Limit if BYMONTHDAY is present; otherwise, special expand
               for MONTHLY.

      Note 2:  Limit if BYYEARDAY or BYMONTHDAY is present; otherwise,
               special expand for WEEKLY if BYWEEKNO present; otherwise,
               special expand for MONTHLY if BYMONTH present; otherwise,
               special expand for YEARLY.
*/

pub mod date;
pub mod recurrence_builder;
pub mod recurrence_positions;
pub mod recurrence_vec;
pub mod weekday;
use serde::{Deserialize, Serialize};

use self::{
    date::Date, recurrence_positions::RecurrencePositions, recurrence_vec::RecurrenceVec,
    weekday::Weekday,
};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct RecurrenceDay {
    // 20th monday or first sunday
    pub recurence: u32,
    pub weekday: Weekday,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recurrence {
    pub frequency: Frequency,

    pub interval: u32,

    pub count: Option<u32>,

    pub until_date: Option<Date>,

    pub week_start: Option<Weekday>,

    pub excluded_dates: Vec<Date>, // TODO: change to set?

    pub recurrences: Vec<RecurrenceDay>,
    pub weekdays: RecurrenceVec<Weekday>,
    pub positions: RecurrencePositions,

    pub hours: RecurrenceVec<u32>,
    pub minutes: RecurrenceVec<u32>,
    pub seconds: RecurrenceVec<u32>,
    pub year_days: RecurrenceVec<u32>,
    pub month_days: RecurrenceVec<u32>,
    pub months: RecurrenceVec<u32>,
}

impl Recurrence {
    pub fn calculate_ocurrences(&self, start_date: Date, end_date: Date) -> Vec<Date> {
        /* Returns all included dates in the recurrence, between start_date and end_date */
        let ending_date = match &self.until_date {
            Some(date) => {
                if date > &end_date {
                    end_date
                } else {
                    *date
                }
            }
            None => end_date,
        };

        let count = self.count.unwrap_or(1000);

        match self.frequency {
            Frequency::Secondly => {
                self.calculate_ocurrences_secondly(start_date, ending_date, count)
            }
            Frequency::Minutely => {
                self.calculate_ocurrences_minutely(start_date, ending_date, count)
            }
            Frequency::Hourly => Vec::new(),  // TODO
            Frequency::Daily => Vec::new(),   // TODO
            Frequency::Weekly => Vec::new(),  // TODO
            Frequency::Monthly => Vec::new(), // TODO
            Frequency::Yearly => Vec::new(),  // TODO
        }
    }

    fn calculate_ocurrences_secondly(
        &self,
        start_date: Date,
        ending_date: Date,
        count: u32,
    ) -> Vec<Date> {
        /* If freq is SECONDLY, we advance every `interval` seconds, but:
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
                skip_to_date =
                    Some(current_date.advance_until_next_available_month_day(&month_days));
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
                let seconds_to_add =
                    self.calculate_interval_to_skip_ocurrence(current_date.seconds_to_date(&date));
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

    fn calculate_ocurrences_minutely(
        &self,
        start_date: Date,
        ending_date: Date,
        count: u32,
    ) -> Vec<Date> {
        /* If freq is MINUTELY, we advance every `interval` minutes, but:
                - Only in the months specified in `months`,
                - Only in the days specified in `month_days` and `year_days` and `weekdays`,
                - Only in the hours specified in `hours`,
            Only accept dates if the minutes are specified in `seconds`,
            If seconds is specified, expand dates to include all those seconds, else use start_date seconds
            If set_pos is specified, within a minute only include those that match the positions in set_pos.
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
                skip_to_date =
                    Some(current_date.advance_until_next_available_month_day(&month_days));
            } else if !weekdays.contains(&current_date.get_weekday()) {
                // Skip to next weekday
                skip_to_date = Some(current_date.advance_until_next_available_weekday(&weekdays));
            } else if !hours.contains(&current_date.get_hour()) {
                // Skip to next hour
                skip_to_date = Some(current_date.advance_until_next_available_hour(&hours));
            }

            if let Some(date) = skip_to_date {
                let minutes_to_add =
                    self.calculate_interval_to_skip_ocurrence(current_date.minutes_to_date(&date));
                current_date = current_date.add_minutes(minutes_to_add);
                continue;
            }

            if minutes.contains(&current_date.get_minute()) {
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
            }

            current_date = current_date.add_minutes(self.interval);
        }
        ocurrences
    }

    fn calculate_interval_to_skip_ocurrence(&self, time: u32) -> u32 {
        let modulo = time % self.interval;
        let result = if modulo == 0 {
            time
        } else {
            time + self.interval - modulo
        };

        if result > 0 {
            result
        } else {
            1
        } // Rounding problems
    }
}
