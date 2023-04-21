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
    pub frequency: Frequency,

    pub interval: u32,

    pub count: Option<u32>,

    pub until_date: Option<Date>,

    pub week_start: Option<Weekday>,

    pub excluded_dates: HashSet<Date>,

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

        let frequency_calculator: Box<dyn RecurrenceFrequencyCalculator> = match self.frequency {
            Frequency::Secondly => Box::new(SecondlyRecurrenceCalculator::new(self, start_date)),
            Frequency::Minutely => Box::new(MinutelyRecurrenceCalculator::new(self, start_date)),
            Frequency::Hourly => Box::new(HourlyRecurrenceCalculator::new(self, start_date)),
            Frequency::Daily => unimplemented!(), // TODO: implement daily recurrence calculator
            Frequency::Weekly => unimplemented!(), // TODO: implement weekly recurrence calculator
            Frequency::Monthly => unimplemented!(), // TODO: implement monthly recurrence calculator
            Frequency::Yearly => unimplemented!(), // TODO: implement yearly recurrence calculator
        };

        RecurrenceCalculator::new(self, frequency_calculator).calculate(start_date, ending_date, count)
    }
}
