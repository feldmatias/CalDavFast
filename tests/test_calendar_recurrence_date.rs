use cal_dav_fast::app::calendar::models::calendar_types::recurrence::{
    date::Date, recurrence_vec::RecurrenceVec, weekday::Weekday,
};
use chrono::{TimeZone, Utc};
use pretty_assertions::assert_eq;

#[test]
fn test_set_month_next_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_skipping_months() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 8, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 10);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_same_month_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 5, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month(5).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 5);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_previous_month_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 5, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month(3).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 3);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_on_invalid_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 30, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_invalid_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month(13);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_next_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
}

#[test]
fn test_set_month_day_skipping_days() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_month_day_same_day_changes_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_month_day_previous_day_changes_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(8).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 8);
}

#[test]
fn test_set_month_day_invalid_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(32);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_december_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(8).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 8);
}

#[test]
fn test_set_month_day_30_february() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(30);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_31_for_30_day_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 4, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(31);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_31_for_31_day_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(31).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 31);
}

#[test]
fn test_set_month_day_29_february() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(29);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_29_february_leap_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2020, 2, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(29).unwrap();

    assert_eq!(new_date.get_year(), 2020);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 29);
}

#[test]
fn test_set_month_day_28_february() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_month_day(28).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 28);
}

#[test]
fn test_set_year_day_next_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
}

#[test]
fn test_set_year_day_skipping_days() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_year_day_next_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(32).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_year_day_skipping_months() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(100).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 4);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_year_day_same_day_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 9, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(9).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 9);
}

#[test]
fn test_set_year_day_previous_day_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 10, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(9).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 9);
}

#[test]
fn test_set_year_day_366() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 8, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(366);

    assert!(new_date.is_none());
}

#[test]
fn test_set_year_day_366_leap_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2020, 1, 8, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(366).unwrap();

    assert_eq!(new_date.get_year(), 2020);
    assert_eq!(new_date.get_month(), 12);
    assert_eq!(new_date.get_month_day(), 31);
}

#[test]
fn test_set_year_day_365() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 8, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(365).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 12);
    assert_eq!(new_date.get_month_day(), 31);
}

#[test]
fn test_set_year_day_invalid() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 8, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_year_day(367);

    assert!(new_date.is_none());
}

#[test]
fn test_set_weekday_next_day() {
    // Sunday
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 3, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 4);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_same_day() {
    // Monday
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 4, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 11);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_previous_day() {
    // Tuesday
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 5, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 11);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_changes_month() {
    // Friday
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_changes_year() {
    // Friday
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 3);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_changes_year_leap_year() {
    // Thursday
    let date = Date::new(
        Utc.with_ymd_and_hms(2020, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 4);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_hour_next_hour() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(1).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 1);
}

#[test]
fn test_set_hour_skipping_hours() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(3).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 3);
}

#[test]
fn test_set_hour_same_hour_changes_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 10, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_previous_hour_changes_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 15, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_changes_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 31, 23, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 23, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(10).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_invalid_hour() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 23, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_hour(24);

    assert!(new_date.is_none());
}

#[test]
fn test_set_minute_next_minute() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(1).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
}

#[test]
fn test_set_minute_skipping_minutes() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(33).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 33);
}

#[test]
fn test_set_minute_same_minute_changes_hour() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 10, 10, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 11);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_previous_minute_changes_hour() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 10, 10, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(9).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 11);
    assert_eq!(new_date.get_minute(), 9);
}

#[test]
fn test_set_minute_changes_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 23, 59, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_changes_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 31, 23, 59, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 23, 59, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(10).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_invalid_minute() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 23, 59, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_minute(60);

    assert!(new_date.is_none());
}

#[test]
fn test_set_second_next_second() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(1).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_set_second_skipping_seconds() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(33).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 33);
}

#[test]
fn test_set_second_same_second_changes_minute() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 10, 10, 10)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
    assert_eq!(new_date.get_minute(), 11);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_previous_second_changes_minute() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 10, 10, 10)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(9).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
    assert_eq!(new_date.get_minute(), 11);
    assert_eq!(new_date.get_second(), 9);
}

#[test]
fn test_set_second_changes_hour() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 10, 59, 59)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 11);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_changes_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 23, 59, 59)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_changes_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 31, 23, 59, 59)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 23, 59, 59)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(10).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_invalid_second() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.set_second(60);

    assert!(new_date.is_none());
}

#[test]
fn test_add_seconds() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.add_seconds(33);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 33);
}

#[test]
fn test_add_seconds_changes_minute() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.add_seconds(70);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_add_seconds_changes_hour() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.add_seconds(3601);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 1);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_add_seconds_changes_day() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.add_seconds(86401);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_add_seconds_changes_month() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.add_seconds(86401);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_add_seconds_changes_year() {
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let new_date = date.add_seconds(86401);

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_advance_until_next_available_month() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 18, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_changes_year() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 3, 18, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_invalid_months() {
    let available = RecurrenceVec::new(vec![1, 2, 3, 13, 18]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 3, 18, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_only_one_available() {
    let available = RecurrenceVec::new(vec![3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 3, 18, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 2);
}

#[test]
fn test_advance_until_next_available_month_day_changes_month() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 3, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_changes_year() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 3, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_invalid_days() {
    let available = RecurrenceVec::new(vec![1, 2, 3, 32, 67]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 3, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_invalid_days_february() {
    let available = RecurrenceVec::new(vec![1, 2, 3, 30]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 3, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_only_one_available() {
    let available = RecurrenceVec::new(vec![3]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 3, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 3);
}

#[test]
fn test_advance_until_next_available_weekday() {
    let available = RecurrenceVec::new(vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 1, 0, 0, 0)
            .earliest()
            .unwrap(), // monday
    );

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 2);
}

#[test]
fn test_advance_until_next_available_weekday_changes_month() {
    let available = RecurrenceVec::new(vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 3, 0, 0, 0)
            .earliest()
            .unwrap(), // wednesday
    );

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 8);
}

#[test]
fn test_advance_until_next_available_weekday_changes_year() {
    let available = RecurrenceVec::new(vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(), // friday
    );

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 3);
}

#[test]
fn test_advance_until_next_available_weekday_only_one_available() {
    let available = RecurrenceVec::new(vec![Weekday::Monday]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 2, 1, 0, 0, 0)
            .earliest()
            .unwrap(), // monday
    );

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 8);
}

#[test]
fn test_advance_until_next_available_year_day() {
    let available = RecurrenceVec::new(vec![1, 40, 333]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 9);
    assert_eq!(result.get_year_day(), 40);
}

#[test]
fn test_advance_until_next_available_year_day_changes_year() {
    let available = RecurrenceVec::new(vec![5, 20, 333]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 5);
    assert_eq!(result.get_year_day(), 5);
}

#[test]
fn test_advance_until_next_available_year_day_only_one_available() {
    let available = RecurrenceVec::new(vec![5]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 5);
    assert_eq!(result.get_year_day(), 5);
}

#[test]
fn test_advance_until_next_available_year_day_invalid_days() {
    let available = RecurrenceVec::new(vec![0, 70, 366]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 12, 31, 0, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 11);
    assert_eq!(result.get_year_day(), 70);
}

#[test]
fn test_advance_until_next_available_hour() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 19);
}

#[test]
fn test_advance_until_next_available_hour_changes_day() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 19, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 2);
    assert_eq!(result.get_hour(), 8);
}

#[test]
fn test_advance_until_next_available_hour_only_one_available() {
    let available = RecurrenceVec::new(vec![8]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 8, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 2);
    assert_eq!(result.get_hour(), 8);
}

#[test]
fn test_advance_until_next_available_hour_invalid_hours() {
    let available = RecurrenceVec::new(vec![0, 24, 25]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 8, 0, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 2);
    assert_eq!(result.get_hour(), 0);
}

#[test]
fn test_advance_until_next_available_minute() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 13, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_minute(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 13);
    assert_eq!(result.get_minute(), 19);
}

#[test]
fn test_advance_until_next_available_minute_changes_hour() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 19, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_minute(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 14);
    assert_eq!(result.get_minute(), 8);
}

#[test]
fn test_advance_until_next_available_minute_only_one_available() {
    let available = RecurrenceVec::new(vec![8]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 8, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_minute(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 14);
    assert_eq!(result.get_minute(), 8);
}

#[test]
fn test_advance_until_next_available_minute_invalid_minutes() {
    let available = RecurrenceVec::new(vec![0, 60, 61]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 8, 0)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_minute(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 14);
    assert_eq!(result.get_minute(), 0);
}

#[test]
fn test_advance_until_next_available_second() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 13, 13)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_second(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 13);
    assert_eq!(result.get_minute(), 13);
    assert_eq!(result.get_second(), 19);
}

#[test]
fn test_advance_until_next_available_second_changes_minute() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 13, 19)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_second(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 13);
    assert_eq!(result.get_minute(), 14);
    assert_eq!(result.get_second(), 8);
}

#[test]
fn test_advance_until_next_available_second_only_one_available() {
    let available = RecurrenceVec::new(vec![8]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 13, 8)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_second(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 13);
    assert_eq!(result.get_minute(), 14);
    assert_eq!(result.get_second(), 8);
}

#[test]
fn test_advance_until_next_available_second_invalid_seconds() {
    let available = RecurrenceVec::new(vec![0, 60, 61]);
    let date = Date::new(
        Utc.with_ymd_and_hms(2021, 1, 1, 13, 13, 8)
            .earliest()
            .unwrap(),
    );

    let result = date.advance_until_next_available_second(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 13);
    assert_eq!(result.get_minute(), 14);
    assert_eq!(result.get_second(), 0);
}
