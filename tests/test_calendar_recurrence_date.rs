use cal_dav_fast::app::calendar::models::calendar_types::recurrence::{
    recurrence_vec::RecurrenceVec, weekday::Weekday,
};
use pretty_assertions::assert_eq;

mod common;
use common::*;

#[test]
fn test_set_month_next_month() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_month(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_skipping_months() {
    let date = create!(Date, year: 2021, month: 1, day: 8);

    let new_date = date.set_month(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 10);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_same_month_changes_year() {
    let date = create!(Date, year: 2021, month: 5, day: 1);

    let new_date = date.set_month(5).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 5);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_previous_month_changes_year() {
    let date = create!(Date, year: 2021, month: 5, day: 1);

    let new_date = date.set_month(3).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 3);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_on_invalid_day() {
    let date = create!(Date, year: 2021, month: 1, day: 30);

    let new_date = date.set_month(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_month_invalid_month() {
    let date = create!(Date);

    let new_date = date.set_month(13);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_next_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_month_day(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
}

#[test]
fn test_set_month_day_skipping_days() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_month_day(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_month_day_same_day_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 10);

    let new_date = date.set_month_day(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_month_day_previous_day_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 10);

    let new_date = date.set_month_day(8).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 8);
}

#[test]
fn test_set_month_day_invalid_day() {
    let date = create!(Date);

    let new_date = date.set_month_day(32);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_december_changes_year() {
    let date = create!(Date, year: 2021, month: 12, day: 10);

    let new_date = date.set_month_day(8).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 8);
}

#[test]
fn test_set_month_day_30_february() {
    let date = create!(Date, year: 2021, month: 2, day: 10);

    let new_date = date.set_month_day(30);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_31_for_30_day_month() {
    let date = create!(Date, year: 2021, month: 4, day: 10);

    let new_date = date.set_month_day(31);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_31_for_31_day_month() {
    let date = create!(Date, year: 2021, month: 1, day: 10);

    let new_date = date.set_month_day(31).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 31);
}

#[test]
fn test_set_month_day_29_february() {
    let date = create!(Date, year: 2021, month: 2);

    let new_date = date.set_month_day(29);

    assert!(new_date.is_none());
}

#[test]
fn test_set_month_day_29_february_leap_year() {
    let date = create!(Date, year: 2020, month: 2);

    let new_date = date.set_month_day(29).unwrap();

    assert_eq!(new_date.get_year(), 2020);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 29);
}

#[test]
fn test_set_month_day_28_february() {
    let date = create!(Date, year: 2021, month: 2);

    let new_date = date.set_month_day(28).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 28);
}

#[test]
fn test_set_year_day_next_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_year_day(2).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
}

#[test]
fn test_set_year_day_skipping_days() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_year_day(10).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_year_day_next_month() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_year_day(32).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
}

#[test]
fn test_set_year_day_skipping_months() {
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let new_date = date.set_year_day(100).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 4);
    assert_eq!(new_date.get_month_day(), 10);
}

#[test]
fn test_set_year_day_same_day_changes_year() {
    let date = create!(Date, year: 2021, month: 1, day: 9);

    let new_date = date.set_year_day(9).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 9);
}

#[test]
fn test_set_year_day_previous_day_changes_year() {
    let date = create!(Date, year: 2021, month: 1, day: 10);

    let new_date = date.set_year_day(9).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 9);
}

#[test]
fn test_set_year_day_366() {
    let date = create!(Date);

    let new_date = date.set_year_day(366);

    assert!(new_date.is_none());
}

#[test]
fn test_set_year_day_366_leap_year() {
    let date = create!(Date, year: 2020);

    let new_date = date.set_year_day(366).unwrap();

    assert_eq!(new_date.get_year(), 2020);
    assert_eq!(new_date.get_month(), 12);
    assert_eq!(new_date.get_month_day(), 31);
}

#[test]
fn test_set_year_day_365() {
    let date = create!(Date, year: 2021);

    let new_date = date.set_year_day(365).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 12);
    assert_eq!(new_date.get_month_day(), 31);
}

#[test]
fn test_set_year_day_invalid() {
    let date = create!(Date);

    let new_date = date.set_year_day(367);

    assert!(new_date.is_none());
}

#[test]
fn test_set_weekday_next_day() {
    let date = create!(Date, year: 2021, month: 1, day: 3); // Sunday

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 4);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_same_day() {
    let date = create!(Date, year: 2021, month: 1, day: 4); // Monday

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 11);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_previous_day() {
    let date = create!(Date, year: 2021, month: 1, day: 5); // Tuesday

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 11);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 31); // Friday

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_changes_year() {
    let date = create!(Date, year: 2021, month: 12, day: 31); // Friday

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 3);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_weekday_changes_year_leap_year() {
    let date = create!(Date, year: 2020, month: 12, day: 31); // Thursday

    let new_date = date.set_weekday(Weekday::Monday).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 4);
    assert_eq!(new_date.get_weekday(), Weekday::Monday);
}

#[test]
fn test_set_hour_next_hour() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0);

    let new_date = date.set_hour(1, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 1);
}

#[test]
fn test_set_hour_skipping_hours() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0);

    let new_date = date.set_hour(3, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 3);
}

#[test]
fn test_set_hour_same_hour_changes_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10);

    let new_date = date.set_hour(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_previous_hour_changes_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 15);

    let new_date = date.set_hour(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 31, hour: 23);

    let new_date = date.set_hour(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_changes_year() {
    let date = create!(Date, year: 2021, month: 12, day: 31, hour: 23);

    let new_date = date.set_hour(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_hour_invalid_hour() {
    let date = create!(Date);

    let new_date = date.set_hour(24, true);

    assert!(new_date.is_none());
}

#[test]
fn test_set_hour_previous_hour_without_consistency() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 15);

    let new_date = date.set_hour(10, false).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
}

#[test]
fn test_set_minute_next_minute() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0);

    let new_date = date.set_minute(1, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
}

#[test]
fn test_set_minute_skipping_minutes() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0);

    let new_date = date.set_minute(33, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 33);
}

#[test]
fn test_set_minute_same_minute_changes_hour() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10, minute: 10);

    let new_date = date.set_minute(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 11);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_previous_minute_changes_hour() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10, minute: 10);

    let new_date = date.set_minute(9, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 11);
    assert_eq!(new_date.get_minute(), 9);
}

#[test]
fn test_set_minute_changes_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 23, minute: 59);

    let new_date = date.set_minute(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 31, hour: 23, minute: 59);

    let new_date = date.set_minute(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_changes_year() {
    let date = create!(Date, year: 2021, month: 12, day: 31, hour: 23, minute: 59);

    let new_date = date.set_minute(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 10);
}

#[test]
fn test_set_minute_invalid_minute() {
    let date = create!(Date);

    let new_date = date.set_minute(60, true);

    assert!(new_date.is_none());
}

#[test]
fn test_set_minute_previous_minute_without_consistency() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 15);

    let new_date = date.set_minute(8, false).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 8);
}

#[test]
fn test_set_second_next_second() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.set_second(1, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_set_second_skipping_seconds() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.set_second(33, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 33);
}

#[test]
fn test_set_second_same_second_changes_minute() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10, minute: 10, second: 10);

    let new_date = date.set_second(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
    assert_eq!(new_date.get_minute(), 11);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_previous_second_changes_minute() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10, minute: 10, second: 10);

    let new_date = date.set_second(9, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
    assert_eq!(new_date.get_minute(), 11);
    assert_eq!(new_date.get_second(), 9);
}

#[test]
fn test_set_second_changes_hour() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10, minute: 59, second: 59);

    let new_date = date.set_second(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 11);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_changes_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 23, minute: 59, second: 59);

    let new_date = date.set_second(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 31, hour: 23, minute: 59, second: 59);

    let new_date = date.set_second(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_changes_year() {
    let date = create!(Date, year: 2021, month: 12, day: 31, hour: 23, minute: 59, second: 59);

    let new_date = date.set_second(10, true).unwrap();

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 10);
}

#[test]
fn test_set_second_invalid_second() {
    let date = create!(Date);

    let new_date = date.set_second(60, true);

    assert!(new_date.is_none());
}

#[test]
fn test_set_second_previous_second_without_consistency() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 10, minute: 10, second: 10);

    let new_date = date.set_second(5, false).unwrap();

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 10);
    assert_eq!(new_date.get_minute(), 10);
    assert_eq!(new_date.get_second(), 5);
}

#[test]
fn test_advance_until_next_available_month() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = create!(Date, year: 2021, month: 2, day: 18);

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_changes_year() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = create!(Date, year: 2021, month: 3, day: 18);

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_invalid_months() {
    let available = RecurrenceVec::new(vec![1, 2, 3, 13, 18]);
    let date = create!(Date, year: 2021, month: 3, day: 18);

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_only_one_available() {
    let available = RecurrenceVec::new(vec![3]);
    let date = create!(Date, year: 2021, month: 3, day: 18);

    let result = date.advance_until_next_available_month(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = create!(Date, year: 2021, month: 2, day: 1);

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 2);
}

#[test]
fn test_advance_until_next_available_month_day_changes_month() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = create!(Date, year: 2021, month: 2, day: 3);

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_changes_year() {
    let available = RecurrenceVec::new(vec![1, 2, 3]);
    let date = create!(Date, year: 2021, month: 12, day: 3);

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_invalid_days() {
    let available = RecurrenceVec::new(vec![1, 2, 3, 32, 67]);
    let date = create!(Date, year: 2021, month: 1, day: 3);

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_invalid_days_february() {
    let available = RecurrenceVec::new(vec![1, 2, 3, 30]);
    let date = create!(Date, year: 2021, month: 2, day: 3);

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 1);
}

#[test]
fn test_advance_until_next_available_month_day_only_one_available() {
    let available = RecurrenceVec::new(vec![3]);
    let date = create!(Date, year: 2021, month: 2, day: 3);

    let result = date.advance_until_next_available_month_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 3);
}

#[test]
fn test_advance_until_next_available_weekday() {
    let available = RecurrenceVec::new(vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday]);
    let date = create!(Date, year: 2021, month: 2, day: 1); // Monday

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 2);
}

#[test]
fn test_advance_until_next_available_weekday_changes_month() {
    let available = RecurrenceVec::new(vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday]);
    let date = create!(Date, year: 2021, month: 2, day: 3); // Wednesday

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 8);
}

#[test]
fn test_advance_until_next_available_weekday_changes_year() {
    let available = RecurrenceVec::new(vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday]);
    let date = create!(Date, year: 2021, month: 12, day: 31); // Friday

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 3);
}

#[test]
fn test_advance_until_next_available_weekday_only_one_available() {
    let available = RecurrenceVec::new(vec![Weekday::Monday]);
    let date = create!(Date, year: 2021, month: 2, day: 1); // Monday

    let result = date.advance_until_next_available_weekday(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 8);
}

#[test]
fn test_advance_until_next_available_year_day() {
    let available = RecurrenceVec::new(vec![1, 40, 333]);
    let date = create!(Date, year: 2021, month: 1, day: 1);

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 2);
    assert_eq!(result.get_month_day(), 9);
    assert_eq!(result.get_year_day(), 40);
}

#[test]
fn test_advance_until_next_available_year_day_changes_year() {
    let available = RecurrenceVec::new(vec![5, 20, 333]);
    let date = create!(Date, year: 2021, month: 12, day: 31);

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 5);
    assert_eq!(result.get_year_day(), 5);
}

#[test]
fn test_advance_until_next_available_year_day_only_one_available() {
    let available = RecurrenceVec::new(vec![5]);
    let date = create!(Date, year: 2021, month: 1, day: 5);

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 5);
    assert_eq!(result.get_year_day(), 5);
}

#[test]
fn test_advance_until_next_available_year_day_invalid_days() {
    let available = RecurrenceVec::new(vec![0, 70, 366]);
    let date = create!(Date, year: 2021, month: 12, day: 17);

    let result = date.advance_until_next_available_year_day(&available);

    assert_eq!(result.get_year(), 2022);
    assert_eq!(result.get_month(), 3);
    assert_eq!(result.get_month_day(), 11);
    assert_eq!(result.get_year_day(), 70);
}

#[test]
fn test_advance_until_next_available_hour() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13);

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 19);
}

#[test]
fn test_advance_until_next_available_hour_changes_day() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 19);

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 2);
    assert_eq!(result.get_hour(), 8);
}

#[test]
fn test_advance_until_next_available_hour_only_one_available() {
    let available = RecurrenceVec::new(vec![8]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 8);

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 2);
    assert_eq!(result.get_hour(), 8);
}

#[test]
fn test_advance_until_next_available_hour_invalid_hours() {
    let available = RecurrenceVec::new(vec![0, 8, 24, 25]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 8);

    let result = date.advance_until_next_available_hour(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 2);
    assert_eq!(result.get_hour(), 0);
}

#[test]
fn test_advance_until_next_available_minute() {
    let available = RecurrenceVec::new(vec![8, 13, 19]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 13);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 19);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 8);

    let result = date.advance_until_next_available_minute(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 14);
    assert_eq!(result.get_minute(), 8);
}

#[test]
fn test_advance_until_next_available_minute_invalid_minutes() {
    let available = RecurrenceVec::new(vec![0, 8, 60, 61]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 8);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 13, second: 13);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 13, second: 19);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 13, second: 8);

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
    let available = RecurrenceVec::new(vec![0, 8, 60, 61]);
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 13, minute: 13, second: 8);

    let result = date.advance_until_next_available_second(&available);

    assert_eq!(result.get_year(), 2021);
    assert_eq!(result.get_month(), 1);
    assert_eq!(result.get_month_day(), 1);
    assert_eq!(result.get_hour(), 13);
    assert_eq!(result.get_minute(), 14);
    assert_eq!(result.get_second(), 0);
}

#[test]
fn test_add_seconds() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

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
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

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
    let date = create!(Date, year: 2021, month: 1, day: 31, hour: 0, minute: 0, second: 0);

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
    let date = create!(Date, year: 2021, month: 12, day: 31, hour: 0, minute: 0, second: 0);

    let new_date = date.add_seconds(86401);

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 1);
}

#[test]
fn test_seconds_to_date_same_date() {
    let date = create!(Date, second: 15);

    let seconds = date.seconds_to_date(&date);

    assert_eq!(seconds, 0);
}

#[test]
fn test_seconds_to_date_same_minute() {
    let date1 = create!(Date, second: 0);
    let date2 = create!(Date, second: 10);

    let seconds = date1.seconds_to_date(&date2);

    assert_eq!(seconds, 10);
}

#[test]
fn test_seconds_to_date_same_hour() {
    let date1 = create!(Date, minute: 0, second: 0);
    let date2 = create!(Date, minute: 10, second: 10);

    let seconds = date1.seconds_to_date(&date2);

    assert_eq!(seconds, 610);
}

#[test]
fn test_seconds_to_date_same_day() {
    let date1 = create!(Date, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, hour: 10, minute: 10, second: 10);

    let seconds = date1.seconds_to_date(&date2);

    assert_eq!(seconds, 36610);
}

#[test]
fn test_seconds_to_date_same_month() {
    let date1 = create!(Date, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, day: 10, hour: 10, minute: 10, second: 10);

    let seconds = date1.seconds_to_date(&date2);

    assert_eq!(seconds, 814210);
}

#[test]
fn test_seconds_to_date_same_year() {
    let date1 = create!(Date, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, month: 10, day: 10, hour: 10, minute: 10, second: 10);

    let seconds = date1.seconds_to_date(&date2);

    assert_eq!(seconds, 24401410);
}

#[test]
fn test_seconds_to_date_different_year() {
    let date1 = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, year: 2022, month: 1, day: 10, hour: 10, minute: 10, second: 10);

    let seconds = date1.seconds_to_date(&date2);

    assert_eq!(seconds, 32350210);
}

#[test]
fn test_add_minutes_changes_minute() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_minutes(1);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_minutes_changes_hour() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_minutes(61);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 1);
    assert_eq!(new_date.get_minute(), 1);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_minutes_changes_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_minutes(1441);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_minutes_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_minutes(44641);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_minutes_changes_year() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_minutes(525601);

    assert_eq!(new_date.get_year(), 2022);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 0);
    assert_eq!(new_date.get_minute(), 1);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_hours_changes_hour() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_hours(1);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 1);
    assert_eq!(new_date.get_hour(), 1);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_hours_changes_day() {
    let date = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);

    let new_date = date.add_hours(25);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 1);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 1);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_add_hours_changes_month() {
    let date = create!(Date, year: 2021, month: 1, day: 31, hour: 0, minute: 0, second: 0);

    let new_date = date.add_hours(50);

    assert_eq!(new_date.get_year(), 2021);
    assert_eq!(new_date.get_month(), 2);
    assert_eq!(new_date.get_month_day(), 2);
    assert_eq!(new_date.get_hour(), 2);
    assert_eq!(new_date.get_minute(), 0);
    assert_eq!(new_date.get_second(), 0);
}

#[test]
fn test_minutes_to_date_same_hour() {
    let date1 = create!(Date, minute: 0, second: 0);
    let date2 = create!(Date, minute: 10, second: 10);

    let minutes = date1.minutes_to_date(&date2);

    assert_eq!(minutes, 10);
}

#[test]
fn test_minutes_to_date_same_day() {
    let date1 = create!(Date, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, hour: 10, minute: 10, second: 10);

    let minutes = date1.minutes_to_date(&date2);

    assert_eq!(minutes, 610);
}

#[test]
fn test_minutes_to_date_same_month() {
    let date1 = create!(Date, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, day: 10, hour: 10, minute: 10, second: 10);

    let minutes = date1.minutes_to_date(&date2);

    assert_eq!(minutes, 13570);
}

#[test]
fn test_minutes_to_date_same_year() {
    let date1 = create!(Date, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, month: 10, day: 10, hour: 10, minute: 10, second: 10);

    let minutes = date1.minutes_to_date(&date2);

    assert_eq!(minutes, 406690);
}

#[test]
fn test_minutes_to_date_different_year() {
    let date1 = create!(Date, year: 2021, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, year: 2022, month: 1, day: 10, hour: 10, minute: 10, second: 10);

    let minutes = date1.minutes_to_date(&date2);

    assert_eq!(minutes, 539170);
}

#[test]
fn test_hours_to_date_same_day() {
    let date1 = create!(Date, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, hour: 10, minute: 10, second: 10);

    let hours = date1.hours_to_date(&date2);

    assert_eq!(hours, 10);
}

#[test]
fn test_hours_to_date_same_month() {
    let date1 = create!(Date, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, day: 3, hour: 10, minute: 10, second: 10);

    let hours = date1.hours_to_date(&date2);

    assert_eq!(hours, 58);
}

#[test]
fn test_hours_to_date_same_year() {
    let date1 = create!(Date, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let date2 = create!(Date, month: 2, day: 3, hour: 10, minute: 10, second: 10);

    let hours = date1.hours_to_date(&date2);

    assert_eq!(hours, 802);
}
