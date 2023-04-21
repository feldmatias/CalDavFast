use cal_dav_fast::app::calendar::models::calendar_types::recurrence::{
    recurrence_builder::RecurrenceBuilder, recurrence_frequency::Frequency,
};
use pretty_assertions::assert_eq;

mod common;
use common::*;

#[test]
fn test_hourly_interval1() {
    /*
    Starts at 12:00:00 PM and ends at 16:10:00 PM on the same day, with a frequency of 1 hour.
    */
    let start_date = create!(Date, hour: 12);
    let end_date = create!(Date, hour: 16, minute: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 5);
    for i in 12..=16 {
        assert_eq!(ocurrences[i - 12], create!(Date, hour: i as u32));
    }
}

#[test]
fn test_hourly_interval1_change_day() {
    /*
    Starts at 11:00:00 PM and ends at 3:10:00 AM on the next day, with a frequency of 1 hour.
    */
    let start_date = create!(Date, day: 4, hour: 23);
    let end_date = create!(Date, day: 5, hour: 3, minute: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 5);
    assert_eq!(ocurrences[0], create!(Date, day: 4, hour: 23));
    assert_eq!(ocurrences[1], create!(Date, day: 5, hour: 0));
    assert_eq!(ocurrences[2], create!(Date, day: 5, hour: 1));
    assert_eq!(ocurrences[3], create!(Date, day: 5, hour: 2));
    assert_eq!(ocurrences[4], create!(Date, day: 5, hour: 3));
}

#[test]
fn test_hourly_interval1_change_month() {
    /*
    Starts at May 31st at 11:00:00 PM and ends at June 1st at 3:10:00 AM, with a frequency of 1 hour.
    */
    let start_date = create!(Date, year: 2021, month: 5, day: 31, hour: 23);
    let end_date = create!(Date, year: 2021, month: 6, day: 1, hour: 3, minute: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 5);
    assert_eq!(ocurrences[0], create!(Date, year: 2021, month: 5, day: 31, hour: 23));
    assert_eq!(ocurrences[1], create!(Date, year: 2021, month: 6, day: 1, hour: 0));
    assert_eq!(ocurrences[2], create!(Date, year: 2021, month: 6, day: 1, hour: 1));
    assert_eq!(ocurrences[3], create!(Date, year: 2021, month: 6, day: 1, hour: 2));
    assert_eq!(ocurrences[4], create!(Date, year: 2021, month: 6, day: 1, hour: 3));
}

#[test]
fn test_hourly_interval1_change_year() {
    /*
    Starts at 31st of December 2021 at 22:59:46 and ends at 1st of January 2022 at 01:04:02.
    */
    let start_date = create!(Date, year: 2021, month: 12, day: 31, hour: 22, minute: 59, second: 46);
    let end_date = create!(Date, year: 2022, month: 1, day: 1, hour: 1, minute: 4, second: 2);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 3);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 2021, month: 12, day: 31, hour: 22, minute: 59, second: 46)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 2021, month: 12, day: 31, hour: 23, minute: 59, second: 46)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 59, second: 46)
    );
}

#[test]
fn test_hourly_interval2() {
    /*
    Starts at 01 and ends at 10 with interval 2.
    */
    let start_date = create!(Date, hour: 1);
    let end_date = create!(Date, hour: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(2)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 5);
    assert_eq!(ocurrences[0], create!(Date, hour: 1));
    assert_eq!(ocurrences[1], create!(Date, hour: 3));
    assert_eq!(ocurrences[2], create!(Date, hour: 5));
    assert_eq!(ocurrences[3], create!(Date, hour: 7));
    assert_eq!(ocurrences[4], create!(Date, hour: 9));
}

#[test]
fn test_hourly_expand_seconds() {
    /*
    Starts at hour 1, end at hour 3, with interval 1.
    Expand seconds are 4, 5, 35.
    */
    let start_date = create!(Date, hour: 1);
    let end_date = create!(Date, hour: 3);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .set_seconds(vec![4, 5, 35])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, hour: 1, second: 4));
    assert_eq!(ocurrences[1], create!(Date, hour: 1, second: 5));
    assert_eq!(ocurrences[2], create!(Date, hour: 1, second: 35));
    assert_eq!(ocurrences[3], create!(Date, hour: 2, second: 4));
    assert_eq!(ocurrences[4], create!(Date, hour: 2, second: 5));
    assert_eq!(ocurrences[5], create!(Date, hour: 2, second: 35));
}

#[test]
fn test_hourly_expand_minutes() {
    /*
    Starts at hour 1, end at hour 3, with interval 1.
    Expand minutes are 4, 5, 35.
    */
    let start_date = create!(Date, hour: 1);
    let end_date = create!(Date, hour: 3);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .set_minutes(vec![4, 5, 35])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, hour: 1, minute: 4));
    assert_eq!(ocurrences[1], create!(Date, hour: 1, minute: 5));
    assert_eq!(ocurrences[2], create!(Date, hour: 1, minute: 35));
    assert_eq!(ocurrences[3], create!(Date, hour: 2, minute: 4));
    assert_eq!(ocurrences[4], create!(Date, hour: 2, minute: 5));
    assert_eq!(ocurrences[5], create!(Date, hour: 2, minute: 35));
}

#[test]
fn test_hourly_expand_minutes_and_seconds() {
    /*
    Starts at hour 1, end at hour 3, with interval 1.
    Expand minutes are 4, 35.
    Expand seconds are 8 and 43.
    */
    let start_date = create!(Date, hour: 1);
    let end_date = create!(Date, hour: 3);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .set_minutes(vec![4, 35])
        .set_seconds(vec![8, 43])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 8);
    assert_eq!(ocurrences[0], create!(Date, hour: 1, minute: 4, second: 8));
    assert_eq!(ocurrences[1], create!(Date, hour: 1, minute: 4, second: 43));
    assert_eq!(ocurrences[2], create!(Date, hour: 1, minute: 35, second: 8));
    assert_eq!(ocurrences[3], create!(Date, hour: 1, minute: 35, second: 43));
    assert_eq!(ocurrences[4], create!(Date, hour: 2, minute: 4, second: 8));
    assert_eq!(ocurrences[5], create!(Date, hour: 2, minute: 4, second: 43));
    assert_eq!(ocurrences[6], create!(Date, hour: 2, minute: 35, second: 8));
    assert_eq!(ocurrences[7], create!(Date, hour: 2, minute: 35, second: 43));
}

#[test]
fn test_hourly_allowed_hours() {
    /*
    Starts at 00:58 and ends at 04:59.
    Allowed hours are 0 and 4
    */
    let start_date = create!(Date, hour: 0, minute: 58);
    let end_date = create!(Date, hour: 4, minute: 59);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_hours(vec![0, 4])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 0, minute: 58));
    assert_eq!(ocurrences[1], create!(Date, hour: 4, minute: 58));
}

#[test]
fn test_hourly_allowed_hours_and_minutes() {
    /*
    Starts at 00:00 and ends at 05:18.
    Allowed minutes are 5, 46.
    Allowed hours are 0, 1, 3.
    */
    let start_date = create!(Date, hour: 0, minute: 0);
    let end_date = create!(Date, hour: 5, minute: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_minutes(vec![5, 46])
        .set_hours(vec![0, 1, 3])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, hour: 0, minute: 5));
    assert_eq!(ocurrences[1], create!(Date, hour: 0, minute: 46));
    assert_eq!(ocurrences[2], create!(Date, hour: 1, minute: 5));
    assert_eq!(ocurrences[3], create!(Date, hour: 1, minute: 46));
    assert_eq!(ocurrences[4], create!(Date, hour: 3, minute: 5));
    assert_eq!(ocurrences[5], create!(Date, hour: 3, minute: 46));
}

#[test]
fn test_hourly_allowed_seconds_and_minutes_and_hours() {
    /*
    Starts at 00:00:00 and ends at 08:05:18.
    Allowed seconds are 5, 46.
    Allowed minutes are 3.
    Allowed hours are 1, 6.
    */
    let start_date = create!(Date, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, hour: 8, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_seconds(vec![5, 46])
        .set_minutes(vec![3])
        .set_hours(vec![1, 6])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 1, minute: 3, second: 5));
    assert_eq!(ocurrences[1], create!(Date, hour: 1, minute: 3, second: 46));
    assert_eq!(ocurrences[2], create!(Date, hour: 6, minute: 3, second: 5));
    assert_eq!(ocurrences[3], create!(Date, hour: 6, minute: 3, second: 46));
}

#[test]
fn test_hourly_allowed_seconds_and_minutes_and_hours_and_month_days() {
    /*
    Starts at day 1st 00:00:00 and ends at day 9th 08:05:18.
    Allowed seconds are 5, 46.
    Allowed minutes are 3.
    Allowed hours are 1.
    Allowed days are 6th.
    */
    let start_date = create!(Date, day: 1, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, day: 9, hour: 8, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_seconds(vec![5, 46])
        .set_minutes(vec![3])
        .set_hours(vec![1])
        .set_month_days(vec![6])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, day: 6, hour: 1, minute: 3, second: 5));
    assert_eq!(ocurrences[1], create!(Date, day: 6, hour: 1, minute: 3, second: 46));
}

#[test]
fn test_hourly_allowed_seconds_and_minutes_and_hours_and_month_days_and_year_days() {
    /*
    Starts at January 1st 00:00:00 and ends at May 9th 08:05:18.
    Allowed seconds are 5, 46.
    Allowed minutes are 3.
    Allowed hours are 1.
    Allowed days are 1, 5, 9, 10, 23.
    Allowed year days are 64 (march 5th), 77 (march 18th) and 91 (april 1st).
    */
    let start_date = create!(Date, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, month: 5, day: 9, hour: 8, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_seconds(vec![5, 46])
        .set_minutes(vec![3])
        .set_hours(vec![1])
        .set_month_days(vec![1, 5, 9, 10, 23])
        .set_year_days(vec![64, 77, 91])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(
        ocurrences[0],
        create!(Date, month: 3, day: 5, hour: 1, minute: 3, second: 5)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, month: 3, day: 5, hour: 1, minute: 3, second: 46)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, month: 4, day: 1, hour: 1, minute: 3, second: 5)
    );
    assert_eq!(
        ocurrences[3],
        create!(Date, month: 4, day: 1, hour: 1, minute: 3, second: 46)
    );
}

#[test]
fn test_hourly_interval4_allowed_hours() {
    /*
    Starts at 0 and ends at 23 with interval 4
    Allowed hours are 0, 4, 6, 7, 8, 9, 11, 13, 18, 19, 20, 23
    */
    let start_date = create!(Date, hour: 0);
    let end_date = create!(Date, hour: 23);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(4)
        .set_until_date(end_date.clone())
        .set_hours(vec![0, 4, 6, 7, 8, 9, 11, 18, 19, 20, 23])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 0));
    assert_eq!(ocurrences[1], create!(Date, hour: 4));
    assert_eq!(ocurrences[2], create!(Date, hour: 8));
    assert_eq!(ocurrences[3], create!(Date, hour: 20));
}

#[test]
fn test_hourly_interval5_one_month_one_day_one_hour_three_minutes() {
    /*
    Starts on January 2022 and ends on January 2024. Allowed months are May.
    Allowed days are 5th. Allowed hours are 1 and 5 and 15. Allowed minutes are 5, 10.
    Frequency is every 5 hours.
    */
    let start_date = create!(Date, year: 2022, month: 5, day: 5, hour: 5, minute: 0, second: 0);
    let end_date = create!(Date, year: 2024, month: 1, day: 1, hour: 0, minute: 0, second: 59);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(5)
        .set_until_date(end_date.clone())
        .set_months(vec![5])
        .set_month_days(vec![5])
        .set_hours(vec![1, 5, 15])
        .set_minutes(vec![5, 10])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 8);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 2022, month: 5, day: 5, hour: 5, minute: 5, second: 0)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 2022, month: 5, day: 5, hour: 5, minute: 10, second: 0)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, year: 2022, month: 5, day: 5, hour: 15, minute: 5, second: 0)
    );
    assert_eq!(
        ocurrences[3],
        create!(Date, year: 2022, month: 5, day: 5, hour: 15, minute: 10, second: 0)
    );
    assert_eq!(
        ocurrences[4],
        create!(Date, year: 2023, month: 5, day: 5, hour: 5, minute: 5, second: 0)
    );
    assert_eq!(
        ocurrences[5],
        create!(Date, year: 2023, month: 5, day: 5, hour: 5, minute: 10, second: 0)
    );
    assert_eq!(
        ocurrences[6],
        create!(Date, year: 2023, month: 5, day: 5, hour: 15, minute: 5, second: 0)
    );
    assert_eq!(
        ocurrences[7],
        create!(Date, year: 2023, month: 5, day: 5, hour: 15, minute: 10, second: 0)
    );
}

#[test]
fn test_hourly_interval5_one_month_one_day_three_hours_start_at_7() {
    /*
    Starts on January 2022 and ends on January 2024. Allowed months are May.
    Allowed days are 5th. Allowed hours are  5, 10, and 12.
    Frequency is every 5 hours. Starts at hour 7
    */
    let start_date = create!(Date, year: 2022, month: 5, day: 5, hour: 7, minute: 17);
    let end_date = create!(Date, year: 2024, month: 1, day: 1, hour: 0, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(5)
        .set_until_date(end_date.clone())
        .set_months(vec![5])
        .set_month_days(vec![5])
        .set_hours(vec![5, 10, 12])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 2022, month: 5, day:5, hour: 12, minute: 17)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 2023, month: 5, day:5, hour: 12, minute: 17)
    );
}

#[test]
fn test_hourly_exluded_dates() {
    /*
    Starts at 00:00 nd ends at 10:10 with a frequency of 1 hour.
    Excluded dates are hours 1, 5, 8, 9, 10.
    */
    let start_date = create!(Date, hour: 0);
    let end_date = create!(Date, hour: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_excluded_dates(
            vec![
                create!(Date, hour: 1),
                create!(Date, hour: 5),
                create!(Date, hour: 8),
                create!(Date, hour: 9),
                create!(Date, hour: 10),
            ]
            .into_iter()
            .collect(),
        )
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, hour: 0));
    assert_eq!(ocurrences[1], create!(Date, hour: 2));
    assert_eq!(ocurrences[2], create!(Date, hour: 3));
    assert_eq!(ocurrences[3], create!(Date, hour: 4));
    assert_eq!(ocurrences[4], create!(Date, hour: 6));
    assert_eq!(ocurrences[5], create!(Date, hour: 7));
}

#[test]
fn test_hourly_interval2_allowed_hours_and_exluded_dates() {
    /*
    Starts at 00:00and ends at 20:20 with a frequency of 2 hours.
    Excluded dates are hours 1, 5, 8, 9, 10, 12.
    Allowed hours are 0, 1, 2, 3, 4, 12, 13, 15, 16
    */
    let start_date = create!(Date, hour: 0);
    let end_date = create!(Date, hour: 20);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(2)
        .set_until_date(end_date.clone())
        .set_hours(vec![0, 1, 2, 3, 4, 12, 13, 15, 16])
        .set_excluded_dates(
            vec![
                create!(Date, hour: 1),
                create!(Date, hour: 5),
                create!(Date, hour: 8),
                create!(Date, hour: 9),
                create!(Date, hour: 10),
                create!(Date, hour: 12),
            ]
            .into_iter()
            .collect(),
        )
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 0));
    assert_eq!(ocurrences[1], create!(Date, hour: 2));
    assert_eq!(ocurrences[2], create!(Date, hour: 4));
    assert_eq!(ocurrences[3], create!(Date, hour: 16));
}

#[test]
fn test_hourly_expand_minutes_before_start_date() {
    /*
    Starts at 05:05 and ends at 06:00, with a frequency of 1 hour.
    Minutes are 1, 2, 10, 20
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 6, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_minutes(vec![1, 2, 10, 20])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 10));
    assert_eq!(ocurrences[1], create!(Date, hour: 5, minute: 20));
}

#[test]
fn test_hour_expand_minutes_before_start_date_with_set_pos() {
    /*
    Starts at 05:05 and ends at 06:00 with a frequency of 1 hour.
    minutes are 1, 2, 10, 20.
    Set pos is -2
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 6, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_minutes(vec![1, 2, 10, 20])
        .set_positions(vec![-2])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 1);
    assert_eq!(ocurrences[0], create!(Date,hour: 5, minute: 10));
}

#[test]
fn test_hourly_set_pos_no_exapanded() {
    /*
    Starts at 05:05 PM and ends at 07:00 PM, with a frequency of 1 hour.
    Set pos is 1
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 7, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 5));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, minute: 5));
}

#[test]
fn test_hourly_set_pos2_no_exapanded() {
    /*
    Starts at 05:05 PM and ends at 07:00 PM, with a frequency of 1 hour.
    Set pos is 1
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 7, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![2])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 5));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, minute: 5));
}

#[test]
fn test_hourly_set_pos_1_expanded_minutes_1() {
    /*
    Starts at 05:05 PM and ends at 07:00 PM, with a frequency of 1 hour.
    Allowed seconds are 45
    Set pos is 1
    */
    let start_date = create!(Date, hour: 5, minute:  5);
    let end_date = create!(Date, hour: 7, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1])
        .set_minutes(vec![45])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 45));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, minute: 45));
}

#[test]
fn test_hourly_set_pos_first_multiple_expanded_minutes() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed minutes are 12, 34, 56
    Set pos is 1
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 7, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1])
        .set_minutes(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 12));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, minute: 12));
}

#[test]
fn test_hourly_set_pos_second_multiple_expanded_multiple() {
    /*
    Starts at 05:05 PM and ends at 07:00 PM, with a frequency of 1 hour.
    Allowed minutes are 12, 34, 56
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 7, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![2])
        .set_minutes(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 34));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, minute: 34));
}

#[test]
fn test_hourly_set_pos_last_multiple_expanded_seconds() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, hour: 5, second: 5);
    let end_date = create!(Date, hour: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![-1])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, second: 56));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, second: 56));
}

#[test]
fn test_hourly_set_pos_negative_multiple_expanded_seconds() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, hour: 5, second: 5);
    let end_date = create!(Date, hour: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![-3])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, second: 12));
}

#[test]
fn test_hourly_set_pos_repeated_multiple_expanded_seconds() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, hour: 5, second: 5);
    let end_date = create!(Date, hour: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1, -3])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, hour: 6, second: 12));
}

#[test]
fn test_hourly_set_pos_invalid_multiple_expanded_seconds() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, hour: 5, second: 5);
    let end_date = create!(Date, hour: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![5])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 0);
}

#[test]
fn test_hourly_set_pos_multiple_ocurrences_multiple_expanded_seconds() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, hour: 5, second: 5);
    let end_date = create!(Date, hour: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1, -1])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, hour: 5, second: 56));
    assert_eq!(ocurrences[2], create!(Date, hour: 6, second: 12));
    assert_eq!(ocurrences[3], create!(Date, hour: 6, second: 56));
}

#[test]
fn test_hourly_set_pos_multiple_ocurrences_multiple_expanded_minutes() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed minutes are 16, 43, 56
    */
    let start_date = create!(Date, hour: 5, minute: 5);
    let end_date = create!(Date, hour: 7, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1, -1])
        .set_minutes(vec![16, 43, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 16));
    assert_eq!(ocurrences[1], create!(Date, hour: 5, minute: 56));
    assert_eq!(ocurrences[2], create!(Date, hour: 6, minute: 16));
    assert_eq!(ocurrences[3], create!(Date, hour: 6, minute: 56));
}

#[test]
fn test_hourly_set_pos_multiple_ocurrences_multiple_expanded_minutes_and_seconds() {
    /*
    Starts at 05:05 and ends at 07:00, with a frequency of 1 hour.
    Allowed minutes are 16, 43.
    Allowed seconds are 12, 47
    */
    let start_date = create!(Date, hour: 5, minute: 5, second: 0);
    let end_date = create!(Date, hour: 7, minute: 0, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_until_date(end_date.clone())
        .set_positions(vec![1, -1])
        .set_minutes(vec![16, 43])
        .set_seconds(vec![12, 47])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 5, minute: 16, second: 12));
    assert_eq!(ocurrences[1], create!(Date, hour: 5, minute: 43, second: 47));
    assert_eq!(ocurrences[2], create!(Date, hour: 6, minute: 16, second: 12));
    assert_eq!(ocurrences[3], create!(Date, hour: 6, minute: 43, second: 47));
}

#[test]
fn test_rfc_example1() {
    /*
    Every 3 hours from 9:00 AM to 5:00 PM on a specific day:

       DTSTART;TZID=America/New_York:19970902T090000
       RRULE:FREQ=HOURLY;INTERVAL=3;UNTIL=19970902T170000Z

       ==> (September 2, 1997 EDT) 09:00,12:00,15:00
    */
    let start_date = create!(Date, year: 1997, month: 9, day: 2, hour: 9, minute: 0, second: 0);
    let end_date = create!(Date, year: 1997, month: 9, day: 2, hour: 17, minute: 0, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Hourly)
        .set_interval(3)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 3);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 1997, month: 9, day: 2, hour: 9, minute: 0, second: 0)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 1997, month: 9, day: 2, hour: 12, minute: 0, second: 0)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, year: 1997, month: 9, day: 2, hour: 15, minute: 0, second: 0)
    );
}
