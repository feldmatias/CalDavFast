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
pub mod recurrence_frequency;
pub mod recurrence_positions;
pub mod recurrence_vec;
pub mod weekday;
use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::{
    date::Date,
    recurrence_frequency::{
        recurrence_calculator::{RecurrenceCalculator, RecurrenceFrequencyCalculator},
        recurrence_hourly::HourlyRecurrenceCalculator,
        recurrence_minutely::MinutelyRecurrenceCalculator,
        recurrence_secondly::SecondlyRecurrenceCalculator,
        Frequency,
    },
    recurrence_positions::RecurrencePositions,
    recurrence_vec::RecurrenceVec,
    weekday::Weekday,
};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct RecurrenceDay {
    // 20th monday or first sunday
    pub recurence: u32,
    pub weekday: Weekday,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recurrence {
    frequency: Frequency,

    interval: u32,

    count: Option<u32>,

    start_date: Date,

    /* Last date of recurrence. Set by user or calculated based on count in Recurrence::new */
    until_date: Option<Date>,

    week_start: Option<Weekday>,

    excluded_dates: HashSet<Date>,

    recurrences: Vec<RecurrenceDay>,
    weekdays: RecurrenceVec<Weekday>,
    positions: RecurrencePositions,

    hours: RecurrenceVec<u32>,
    minutes: RecurrenceVec<u32>,
    seconds: RecurrenceVec<u32>,
    year_days: RecurrenceVec<u32>,
    month_days: RecurrenceVec<u32>,
    months: RecurrenceVec<u32>,
}

impl Recurrence {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        frequency: Frequency,
        interval: u32,
        count: Option<u32>,
        start_date: Date,
        until_date: Option<Date>,
        week_start: Option<Weekday>,
        excluded_dates: HashSet<Date>,
        positions: RecurrencePositions,
        recurrences: Vec<RecurrenceDay>,
        weekdays: RecurrenceVec<Weekday>,
        hours: RecurrenceVec<u32>,
        minutes: RecurrenceVec<u32>,
        seconds: RecurrenceVec<u32>,
        year_days: RecurrenceVec<u32>,
        month_days: RecurrenceVec<u32>,
        months: RecurrenceVec<u32>,
    ) -> Result<Self, String> {
        // TODO tests
        if count.is_none() && until_date.is_none() {
            return Err("count or until_date must be set".to_string());
        }

        let mut recurrence = Self {
            frequency,
            interval,
            count,
            start_date,
            until_date,
            week_start,
            excluded_dates,
            recurrences,
            weekdays,
            positions,
            hours,
            minutes,
            seconds,
            year_days,
            month_days,
            months,
        };

        recurrence.until_date = Some(recurrence.calculate_end_date());

        Ok(recurrence)
    }

    pub fn calculate_ocurrences(&self, start_date: Option<Date>, end_date: Option<Date>) -> Vec<Date> {
        /* Returns all included dates in the recurrence, between start_date and end_date */

        // TODO: always use self.start_date and ignore ocurrences before start_date, to avoid problems with intervals
        let starting_date = match start_date {
            Some(date) => {
                if date > self.start_date {
                    date
                } else {
                    self.start_date
                }
            }
            None => self.start_date,
        };

        // ending date is none if both self.until_date and end_date are none
        // if self.until_date is none, ending_date is end_date
        // if end_date is none, ending_date is self.until_date
        // if both are not none, ending_date is the earliest of the two
        let ending_date = match (self.until_date, end_date) {
            (None, None) => Date::new(
                DateTime::parse_from_rfc3339("5000-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            (None, Some(date)) => date,
            (Some(date), None) => date,
            (Some(date1), Some(date2)) => {
                if date1 < date2 {
                    date1
                } else {
                    date2
                }
            }
        };

        let count = self.count.unwrap_or(9999999);

        let frequency_calculator: Box<dyn RecurrenceFrequencyCalculator> = match self.frequency {
            Frequency::Secondly => Box::new(SecondlyRecurrenceCalculator::new(self, starting_date)),
            Frequency::Minutely => Box::new(MinutelyRecurrenceCalculator::new(self, starting_date)),
            Frequency::Hourly => Box::new(HourlyRecurrenceCalculator::new(self, starting_date)),
            Frequency::Daily => unimplemented!(), // TODO: implement daily recurrence calculator
            Frequency::Weekly => unimplemented!(), // TODO: implement weekly recurrence calculator
            Frequency::Monthly => unimplemented!(), // TODO: implement monthly recurrence calculator
            Frequency::Yearly => unimplemented!(), // TODO: implement yearly recurrence calculator
        };

        RecurrenceCalculator::new(self, frequency_calculator).calculate(starting_date, ending_date, count)
    }

    fn calculate_end_date(&self) -> Date {
        // TODO
        // If we have count, we need to calculate the end date, if not, we are sure we have until_date and return that
        // Also modify calculation to avoid storing in memory all the dates, since we only need the last one
        // Also, add duration to last ocurrence, since the method returns start dates of ocurrences, not end dates
        self.until_date.unwrap_or(Date::new(
            DateTime::parse_from_rfc3339("5000-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        ))
    }
}
