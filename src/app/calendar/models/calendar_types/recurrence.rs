/*
The RRULE attribute is used to specify a recurrence rule for a recurring event in iCalendar. The value of the RRULE attribute is a string that consists of a set of name-value pairs, where the name indicates a parameter of the recurrence rule, and the value provides the specific value for that parameter.

Here are the possible parameters that can be used in an RRULE, along with their possible values:

FREQ: Indicates the frequency of the recurrence, and can be set to SECONDLY, MINUTELY, HOURLY, DAILY, WEEKLY, MONTHLY, or YEARLY.

INTERVAL: Specifies the interval between each occurrence of the event, in terms of the specified frequency. For example, if FREQ is set to WEEKLY and INTERVAL is set to 2, the event would occur every two weeks.

COUNT: Specifies the number of times the event should recur. If this parameter is not specified, the event will recur indefinitely.

UNTIL: Specifies the date and time at which the event should stop recurring. This value should be a date-time value in UTC format.

BYDAY: Specifies the day or days of the week on which the event should occur. This parameter should be used only for weekly or monthly events. Possible values include MO (Monday), TU (Tuesday), WE (Wednesday), TH (Thursday), FR (Friday), SA (Saturday), and SU (Sunday).

BYMONTHDAY: Specifies the day of the month on which the event should occur. This parameter should be used only for monthly events.

BYMONTH: Specifies the month or months in which the event should occur. This parameter should be used only for yearly events, and the value should be an integer between 1 and 12.
*/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurrenceDay {
    // 20th monday or first sunday
    recurence: u32,
    weekday: Weekday,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExcludedDate {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recurrence {
    frequency: Frequency,

    interval: u32,

    count: Option<u32>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    until_date: DateTime<Utc>,

    week_start: Option<Weekday>,

    excluded_dates: Vec<ExcludedDate>,

    recurrences: Vec<RecurrenceDay>,
    months: Vec<u32>,
    weekdays: Vec<Weekday>,
    year_days: Vec<u32>,
    month_days: Vec<u32>,
    set_pos: Option<i32>,
    hours: Vec<u32>,
    minutes: Vec<u32>,
}

impl Recurrence {
    pub fn calculate_included_dates(
        &self,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Vec<DateTime<Utc>> {
        /* Returns all included dates in the recurrence, between start_date and end_date */
        vec![]
    }
}
