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
pub mod recurrence_vec;
pub mod weekday;
use serde::{Deserialize, Serialize};

use self::{date::Date, recurrence_vec::RecurrenceVec, weekday::Weekday};

#[derive(Debug, Serialize, Deserialize)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize)]
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

    pub excluded_dates: Vec<Date>,

    pub recurrences: Vec<RecurrenceDay>,
    pub weekdays: RecurrenceVec<Weekday>,
    pub set_pos: RecurrenceVec<i32>,

    pub hours: RecurrenceVec<u32>,
    pub minutes: RecurrenceVec<u32>,
    pub seconds: RecurrenceVec<u32>,
    pub year_days: RecurrenceVec<u32>,
    pub month_days: RecurrenceVec<u32>,
    pub months: RecurrenceVec<u32>,
}

impl Recurrence {
    pub fn calculate_included_dates(&self, start_date: Date, end_date: Date) -> Vec<Date> {
        /* Returns all included dates in the recurrence, between start_date and end_date */
        /* If freq is SECONDLY, we advance every `interval` seconds, but:
                - Only in the months specified in `months`,
                - Only in the days specified in `month_days` and `year_days` and `weekdays`,
                - Only in the hours specified in `hours`,
                - Only in the minutes specified in `minutes`,
                - Only in the seconds specified in `seconds`,
        */
        let mut included_dates = Vec::new();

        let mut current_date = start_date;
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

        let mut count = self.count.unwrap_or(1000);

        // SECONDLY freq
        match self.frequency {
            Frequency::Secondly => {
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
                    if !months.contains(&current_date.get_month()) {
                        // Skip to next month

                        // TODO: move this to Date object
                        // pub fn advance_until_next_available_month(&self, available: &RecurrenceVec<u32>) -> Date {
                        let mut next_month = months.get_next(&current_date.get_month());
                        let mut new_date = current_date.set_month(next_month);
                        while new_date.is_none() {
                            next_month = months.get_next(&next_month);
                            new_date = current_date.set_month(next_month);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }
                    if !year_days.contains(&current_date.get_year_day()) {
                        // Skip to next year day
                        let mut next_year_day = year_days.get_next(&current_date.get_year_day());
                        let mut new_date = current_date.set_year_day(next_year_day);
                        while new_date.is_none() {
                            next_year_day = year_days.get_next(&next_year_day);
                            new_date = current_date.set_year_day(next_year_day);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }
                    if !month_days.contains(&current_date.get_month_day()) {
                        // Skip to next month day
                        let mut next_month_day = month_days.get_next(&current_date.get_month_day());
                        let mut new_date = current_date.set_month_day(next_month_day);
                        while new_date.is_none() {
                            next_month_day = month_days.get_next(&next_month_day);
                            new_date = current_date.set_month_day(next_month_day);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }
                    if !weekdays.contains(&current_date.get_weekday()) {
                        // Skip to next weekday
                        let mut next_weekday = weekdays.get_next(&current_date.get_weekday());
                        let mut new_date = current_date.set_weekday(next_weekday);
                        while new_date.is_none() {
                            next_weekday = weekdays.get_next(&next_weekday);
                            new_date = current_date.set_weekday(next_weekday);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }
                    if !hours.contains(&current_date.get_hour()) {
                        // Skip to next hour
                        let mut next_hour = hours.get_next(&current_date.get_hour());
                        let mut new_date = current_date.set_hour(next_hour);
                        while new_date.is_none() {
                            next_hour = hours.get_next(&next_hour);
                            new_date = current_date.set_hour(next_hour);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }
                    if !minutes.contains(&current_date.get_minute()) {
                        // Skip to next minute
                        let mut next_minute = minutes.get_next(&current_date.get_minute());
                        let mut new_date = current_date.set_minute(next_minute);
                        while new_date.is_none() {
                            next_minute = minutes.get_next(&next_minute);
                            new_date = current_date.set_minute(next_minute);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }
                    if !seconds.contains(&current_date.get_second()) {
                        // Skip to next second
                        let mut next_second = seconds.get_next(&current_date.get_second());
                        let mut new_date = current_date.set_second(next_second);
                        while new_date.is_none() {
                            next_second = seconds.get_next(&next_second);
                            new_date = current_date.set_second(next_second);
                        }
                        current_date = new_date.unwrap();
                        continue;
                    }

                    count -= 1;
                    included_dates.push(current_date);

                    current_date = current_date.add_seconds(self.interval);
                }
            }
            Frequency::Minutely => (), // TODO
            Frequency::Hourly => (),   // TODO
            Frequency::Daily => (),    // TODO
            Frequency::Weekly => (),   // TODO
            Frequency::Monthly => (),  // TODO
            Frequency::Yearly => (),   // TODO
        }

        included_dates
    }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn test_secondly() {
        let start_date = Date::new(
            Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
                .earliest()
                .unwrap(),
        );
        let end_date = Date::new(
            Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
                .earliest()
                .unwrap(),
        );

        let recurrence = Recurrence {
            frequency: Frequency::Secondly,
            interval: 5,
            until_date: Some(end_date.clone()),
            week_start: None,
            weekdays: RecurrenceVec::default(),
            hours: RecurrenceVec::new(vec![5]),
            minutes: RecurrenceVec::new(vec![5]),
            seconds: RecurrenceVec::new(vec![5, 10, 12]),
            month_days: RecurrenceVec::new(vec![5]),
            months: RecurrenceVec::new(vec![5]),
            year_days: RecurrenceVec::default(),
            count: None,
            excluded_dates: Vec::default(),
            set_pos: RecurrenceVec::default(),
            recurrences: Vec::default(),
        };

        let included_dates = recurrence.calculate_included_dates(start_date, end_date);

        assert_eq!(included_dates.len(), 4);
        assert_eq!(
            included_dates[0],
            Utc.with_ymd_and_hms(2022, 5, 5, 5, 5, 5)
                .earliest()
                .unwrap()
        );
        assert_eq!(
            included_dates[1],
            Utc.with_ymd_and_hms(2022, 5, 5, 5, 5, 10)
                .earliest()
                .unwrap()
        );
        assert_eq!(
            included_dates[2],
            Utc.with_ymd_and_hms(2023, 5, 5, 5, 5, 5)
                .earliest()
                .unwrap()
        );
        assert_eq!(
            included_dates[3],
            Utc.with_ymd_and_hms(2023, 5, 5, 5, 5, 10)
                .earliest()
                .unwrap()
        );
    }
}
