use cal_dav_fast::app::calendar::models::calendar_types::recurrence::{
    date::Date, weekday::Weekday,
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

    let new_date = date.set_month(2);

    assert!(new_date.is_none());
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
