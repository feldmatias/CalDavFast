use cal_dav_fast::app::calendar::models::calendar_types::recurrence::date::Date;
use chrono::{TimeZone, Utc};
use factori::*;

factori!(Date, {
    default {
        year: i32 = 2021,
        month: u32 = 1,
        day: u32 = 1,
        hour: u32 = 0,
        minute: u32 = 0,
        second: u32 = 0,
    }

    builder {
        Date::new(
            Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
                .earliest()
                .unwrap(),
        )
    }

});
