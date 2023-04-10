use cal_dav_fast::app::calendar::models::calendar_types::recurrence::{
    recurrence_builder::RecurrenceBuilder, weekday::Weekday, Frequency,
};
use pretty_assertions::assert_eq;

mod common;
use common::*;

#[test]
fn test_minutely_interval1() {
    /*
    Starts at 12:00:00 PM and ends at 12:10:00 PM on the same day, with a frequency of 1 minute.
    */
    let start_date = create!(Date, minute: 0);
    let end_date = create!(Date, minute: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 11);
    for i in 0..=10 {
        assert_eq!(ocurrences[i], create!(Date, minute: i as u32));
    }
}

#[test]
fn test_minutely_interval1_change_hour() {
    /*
    Starts at 12:58:58 and ends at 12:01:01.
    */
    let start_date = create!(Date, hour: 0, minute: 58, second: 58);
    let end_date = create!(Date, hour: 1, minute: 1, second: 1);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 3);
    assert_eq!(
        ocurrences[0],
        create!(Date, hour: 0, minute: 58, second: 58)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, hour: 0, minute: 59, second: 58)
    );
    assert_eq!(ocurrences[2], create!(Date, hour: 1, minute: 0, second: 58));
}

#[test]
fn test_minutely_interval1_change_day() {
    /*
    Starts at 23:58:37 and ends at 00:01:48.
    */
    let start_date = create!(Date, day: 4, hour: 23, minute: 58, second: 37);
    let end_date = create!(Date, day: 5, hour: 0, minute: 1, second: 48);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(
        ocurrences[0],
        create!(Date, day: 4, hour: 23, minute: 58, second: 37)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, day: 4, hour: 23, minute: 59, second: 37)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, day: 5, hour: 0, minute: 0, second: 37)
    );
    assert_eq!(
        ocurrences[3],
        create!(Date, day: 5, hour: 0, minute: 1, second: 37)
    );
}

#[test]
fn test_minutely_interval1_change_month() {
    /*
    Starts at 30th of April 2021 at 23:57:58 and ends at 1st of May 2021 at 00:01:02.
    */
    let start_date = create!(Date, month: 4, day: 30, hour: 23, minute: 57, second: 58);
    let end_date = create!(Date, month: 5, day: 1, hour: 0, minute: 1, second: 2);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(
        ocurrences[0],
        create!(Date, month: 4, day: 30, hour: 23, minute: 57, second: 58)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, month: 4, day: 30, hour: 23, minute: 58, second: 58)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, month: 4, day: 30, hour: 23, minute: 59, second: 58)
    );
    assert_eq!(
        ocurrences[3],
        create!(Date, month: 5, day: 1, hour: 0, minute: 0, second: 58)
    );
}

#[test]
fn test_minutely_interval1_change_year() {
    /*
    Starts at 31st of December 2021 at 23:59:46 and ends at 1st of January 2022 at 00:04:02.
    */
    let start_date =
        create!(Date, year: 2021, month: 12, day: 31, hour: 23, minute: 59, second: 46);
    let end_date = create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 4, second: 2);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 5);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 2021, month: 12, day: 31, hour: 23, minute: 59, second: 46)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 0, second: 46)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 1, second: 46)
    );
    assert_eq!(
        ocurrences[3],
        create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 2, second: 46)
    );
    assert_eq!(
        ocurrences[4],
        create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 3, second: 46)
    );
}

#[test]
fn test_minutely_interval2() {
    /*
    Starts at 00:01:04 and ends at 00:11:11 with interval 2.
    */
    let start_date = create!(Date, minute: 1, second: 4);
    let end_date = create!(Date, minute: 11, second: 11);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(2)
        .set_until_date(end_date.clone())
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, minute: 1, second: 4));
    assert_eq!(ocurrences[1], create!(Date, minute: 3, second: 4));
    assert_eq!(ocurrences[2], create!(Date, minute: 5, second: 4));
    assert_eq!(ocurrences[3], create!(Date, minute: 7, second: 4));
    assert_eq!(ocurrences[4], create!(Date, minute: 9, second: 4));
    assert_eq!(ocurrences[5], create!(Date, minute: 11, second: 4));
}

#[test]
fn test_minutely_expand_seconds() {
    /*
    Starts at minute 1, end at minute 3, with interval 1.
    Expand seconds are 4, 5, 35.
    */
    let start_date = create!(Date, minute: 1, second: 4);
    let end_date = create!(Date, minute: 3, second: 15);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(1)
        .set_until_date(end_date.clone())
        .set_seconds(vec![4, 5, 35])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 8);
    assert_eq!(ocurrences[0], create!(Date, minute: 1, second: 4));
    assert_eq!(ocurrences[1], create!(Date, minute: 1, second: 5));
    assert_eq!(ocurrences[2], create!(Date, minute: 1, second: 35));
    assert_eq!(ocurrences[3], create!(Date, minute: 2, second: 4));
    assert_eq!(ocurrences[4], create!(Date, minute: 2, second: 5));
    assert_eq!(ocurrences[5], create!(Date, minute: 2, second: 35));
    assert_eq!(ocurrences[6], create!(Date, minute: 3, second: 4));
    assert_eq!(ocurrences[7], create!(Date, minute: 3, second: 5));
}

#[test]
fn test_minutely_allowed_minutes() {
    /*
    Starts at 00:58 and ends at 04:59.
    Allowed minutes are 0 and 4
    */
    let start_date = create!(Date, minute: 0, second: 58);
    let end_date = create!(Date, minute: 4, second: 59);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_minutes(vec![0, 4])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 0, second: 58));
    assert_eq!(ocurrences[1], create!(Date, minute: 4, second: 58));
}

#[test]
fn test_minutely_allowed_hours() {
    /*
    Starts at 00:00:58 and ends at 04:45:01.
    Allowed minutes are 2 and 43.
    Allowed hours are 0 and 4.
    */
    let start_date = create!(Date, hour: 0, minute: 0, second: 58);
    let end_date = create!(Date, hour: 4, minute: 45, second: 1);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_hours(vec![0, 4])
        .set_minutes(vec![2, 43])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 0, minute: 2, second: 58));
    assert_eq!(
        ocurrences[1],
        create!(Date, hour: 0, minute: 43, second: 58)
    );
    assert_eq!(ocurrences[2], create!(Date, hour: 4, minute: 2, second: 58));
    assert_eq!(
        ocurrences[3],
        create!(Date, hour: 4, minute: 43, second: 58)
    );
}

#[test]
fn test_minutely_allowed_seconds_and_minutes() {
    /*
    Starts at 00:00:00 and ends at 00:05:18.
    Allowed seconds are 5, 46.
    Allowed minutes are 0, 1, 3.
    */
    let start_date = create!(Date, minute: 0, second: 0);
    let end_date = create!(Date, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_seconds(vec![5, 46])
        .set_minutes(vec![0, 1, 3])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, minute: 0, second: 5));
    assert_eq!(ocurrences[1], create!(Date, minute: 0, second: 46));
    assert_eq!(ocurrences[2], create!(Date, minute: 1, second: 5));
    assert_eq!(ocurrences[3], create!(Date, minute: 1, second: 46));
    assert_eq!(ocurrences[4], create!(Date, minute: 3, second: 5));
    assert_eq!(ocurrences[5], create!(Date, minute: 3, second: 46));
}

#[test]
fn test_minutely_allowed_seconds_and_minutes_and_hours() {
    /*
    Starts at 00:00:00 and ends at 08:05:18.
    Allowed seconds are 5, 46.
    Allowed minutes are 3.
    Allowed hours are 1, 6.
    */
    let start_date = create!(Date, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, hour: 8, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
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
fn test_minutely_allowed_seconds_and_minutes_and_hours_and_month_days() {
    /*
    Starts at day 1st 00:00:00 and ends at day 9th 08:05:18.
    Allowed seconds are 5, 46.
    Allowed minutes are 3.
    Allowed hours are 1.
    Allowed days are 6th.
    */
    let start_date = create!(Date, day: 1, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, day: 9, hour: 8, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_seconds(vec![5, 46])
        .set_minutes(vec![3])
        .set_hours(vec![1])
        .set_month_days(vec![6])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(
        ocurrences[0],
        create!(Date, day: 6, hour: 1, minute: 3, second: 5)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, day: 6, hour: 1, minute: 3, second: 46)
    );
}

#[test]
fn test_minutely_allowed_seconds_and_minutes_and_hours_and_month_days_and_year_days() {
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

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
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
fn test_minutely_allowed_seconds_and_minutes_and_hours_and_month_days_and_weekdays() {
    /*
    Starts at January 1st 00:00:00 and ends at May 9th 08:05:18.
    Allowed seconds are 46.
    Allowed minutes are 3.
    Allowed hours are 1.
    Allowed days are 1 (monday), 5 (friday), 9 (tuesday), 10 (wednesday), 23 (tuesday).
    Allowed month is March.
    Allowed weekdays are monday and friday.
    */
    let start_date = create!(Date, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, month: 5, day: 9, hour: 8, minute: 5, second: 18);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_seconds(vec![46])
        .set_minutes(vec![3])
        .set_hours(vec![1])
        .set_month_days(vec![1, 5, 9, 10, 23])
        .set_months(vec![3])
        .set_weekdays(vec![Weekday::Monday, Weekday::Friday])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(
        ocurrences[0],
        create!(Date, month: 3, day: 1, hour: 1, minute: 3, second: 46)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, month: 3, day: 5, hour: 1, minute: 3, second: 46)
    );
}

#[test]
fn test_minutely_interval5_allowed_minutes() {
    /*
    Starts at 00:01:00 and ends at 00:59:59 with interval 5.
    Allowed minutes are 0, 5, 6, 7, 8, 9, 11, 18, 19, 20, 25, 32, 37, 38, 39, 55
    */
    let start_date = create!(Date, minute: 0);
    let end_date = create!(Date, minute: 59);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(5)
        .set_until_date(end_date.clone())
        .set_minutes(vec![
            0, 5, 6, 7, 8, 9, 11, 18, 19, 20, 25, 32, 37, 38, 39, 55,
        ])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 5);
    assert_eq!(ocurrences[0], create!(Date, minute: 0));
    assert_eq!(ocurrences[1], create!(Date, minute: 5));
    assert_eq!(ocurrences[2], create!(Date, minute: 20));
    assert_eq!(ocurrences[3], create!(Date, minute: 25));
    assert_eq!(ocurrences[4], create!(Date, minute: 55));
}

#[test]
fn test_minutely_interval5_one_month_one_day_one_hour_three_minutes() {
    /*
    Starts on January 2022 and ends on January 2024. Allowed months are May.
    Allowed days are 5th. Allowed hours are 5 AM. Allowed minutes are 5, 10, and 12.
    Frequency is every 5 seconds.
    */
    let start_date = create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 0, second: 0);
    let end_date = create!(Date, year: 2024, month: 1, day: 1, hour: 0, minute: 0, second: 59);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(5)
        .set_until_date(end_date.clone())
        .set_months(vec![5])
        .set_month_days(vec![5])
        .set_hours(vec![5])
        .set_minutes(vec![5, 10, 12])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 2022, month: 5, day:5, hour: 5, minute: 5)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 2022, month: 5, day:5, hour: 5, minute: 10)
    );
    assert_eq!(
        ocurrences[2],
        create!(Date, year: 2023, month: 5, day:5, hour: 5, minute: 5)
    );
    assert_eq!(
        ocurrences[3],
        create!(Date, year: 2023, month: 5, day:5, hour: 5, minute: 10)
    );
}

#[test]
fn test_minutely_interval5_one_month_one_day_one_hour_three_minutes_start_at_7() {
    /*
    Starts on January 2022 and ends on January 2024. Allowed months are May.
    Allowed days are 5th. Allowed hours are 5 AM. Allowed minutes are 5, 10, and 12.
    Frequency is every 5 seconds. Starts at second 7
    */
    let start_date = create!(Date, year: 2022, month: 1, day: 1, hour: 0, minute: 7, second: 0);
    let end_date = create!(Date, year: 2024, month: 1, day: 1, hour: 0, minute: 0, second: 59);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(5)
        .set_until_date(end_date.clone())
        .set_months(vec![5])
        .set_month_days(vec![5])
        .set_hours(vec![5])
        .set_minutes(vec![5, 10, 12])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(
        ocurrences[0],
        create!(Date, year: 2022, month: 5, day:5, hour: 5, minute: 12)
    );
    assert_eq!(
        ocurrences[1],
        create!(Date, year: 2023, month: 5, day:5, hour: 5, minute: 12)
    );
}

#[test]
fn test_minutely_exluded_dates() {
    /*
    Starts at 12:00:00 PM and ends at 12:10:10 PM, with a frequency of 1 minute.
    Excluded dates are minutes 1, 5, 8, 9, 10.
    */
    let start_date = create!(Date, minute: 0);
    let end_date = create!(Date, minute: 10);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_excluded_dates(
            vec![
                create!(Date, minute: 1),
                create!(Date, minute: 5),
                create!(Date, minute: 8),
                create!(Date, minute: 9),
                create!(Date, minute: 10),
            ]
            .into_iter()
            .collect(),
        )
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, minute: 0));
    assert_eq!(ocurrences[1], create!(Date, minute: 2));
    assert_eq!(ocurrences[2], create!(Date, minute: 3));
    assert_eq!(ocurrences[3], create!(Date, minute: 4));
    assert_eq!(ocurrences[4], create!(Date, minute: 6));
    assert_eq!(ocurrences[5], create!(Date, minute: 7));
}

#[test]
fn test_minutely_interval2_allowed_minutes_and_exluded_dates() {
    /*
    Starts at 12:00:00 PM and ends at 12:20:20 PM, with a frequency of 2 minutes.
    Excluded dates are minutes 1, 5, 8, 9, 10, 12.
    Allowed minutes are 0, 1, 2, 3, 4, 12, 13, 15, 16
    */
    let start_date = create!(Date, minute: 0);
    let end_date = create!(Date, minute: 20);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(2)
        .set_until_date(end_date.clone())
        .set_minutes(vec![0, 1, 2, 3, 4, 12, 13, 15, 16])
        .set_excluded_dates(
            vec![
                create!(Date, minute: 1),
                create!(Date, minute: 5),
                create!(Date, minute: 8),
                create!(Date, minute: 9),
                create!(Date, minute: 10),
                create!(Date, minute: 12),
            ]
            .into_iter()
            .collect(),
        )
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, minute: 0));
    assert_eq!(ocurrences[1], create!(Date, minute: 2));
    assert_eq!(ocurrences[2], create!(Date, minute: 4));
    assert_eq!(ocurrences[3], create!(Date, minute: 16));
}

#[test]
fn test_minutely_expand_seconds_before_start_date() {
    /*
    Starts at 12:05:05 PM and ends at 12:06:00 PM, with a frequency of 1 minute.
    Seconds are 1, 2, 10, 20
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 6, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_seconds(vec![1, 2, 10, 20])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 10));
    assert_eq!(ocurrences[1], create!(Date, minute: 5, second: 20));
}

#[test]
fn test_minutely_expand_seconds_before_start_date_with_set_pos() {
    /*
    Starts at 12:05:05 PM and ends at 12:06:00 PM, with a frequency of 1 minute.
    Seconds are 1, 2, 10, 20.
    Set pos is -2
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 6, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_seconds(vec![1, 2, 10, 20])
        .set_positions(vec![-2])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 1);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 10));
}

#[test]
fn test_rfc_every_15_minutes_6_ocurrences() {
    /*
    Every 15 minutes for 6 occurrences:

       DTSTART;TZID=America/New_York:19970902T090000
       RRULE:FREQ=MINUTELY;INTERVAL=15;COUNT=6

       ==> (September 2, 1997 EDT) 09:00,09:15,09:30,09:45,10:00,10:15
    */
    let start_date = create!(Date, hour: 9, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(15)
        .set_count(6)
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, start_date.set_month(12).unwrap());

    assert_eq!(ocurrences.len(), 6);
    assert_eq!(ocurrences[0], create!(Date, hour: 9, minute: 0));
    assert_eq!(ocurrences[1], create!(Date, hour: 9, minute: 15));
    assert_eq!(ocurrences[2], create!(Date, hour: 9, minute: 30));
    assert_eq!(ocurrences[3], create!(Date, hour: 9, minute: 45));
    assert_eq!(ocurrences[4], create!(Date, hour: 10, minute: 0));
    assert_eq!(ocurrences[5], create!(Date, hour: 10, minute: 15));
}

#[test]
fn test_rfc_every_hour_and_a_half_for_4_ocurrences() {
    /*
    Every hour and a half for 4 occurrences:

       DTSTART;TZID=America/New_York:19970902T090000
       RRULE:FREQ=MINUTELY;INTERVAL=90;COUNT=4

       ==> (September 2, 1997 EDT) 09:00,10:30,12:00,13:30
    */
    let start_date = create!(Date, hour: 9, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(90)
        .set_count(4)
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, start_date.set_month(12).unwrap());

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, hour: 9, minute: 0));
    assert_eq!(ocurrences[1], create!(Date, hour: 10, minute: 30));
    assert_eq!(ocurrences[2], create!(Date, hour: 12, minute: 0));
    assert_eq!(ocurrences[3], create!(Date, hour: 13, minute: 30));
}

#[test]
fn test_rfc_every_20_minutes_from_9_to_4_40_every_day() {
    /*
    Every 20 minutes from 9:00 AM to 4:40 PM every day:

       DTSTART;TZID=America/New_York:19970902T090000
       RRULE:FREQ=MINUTELY;INTERVAL=20;BYHOUR=9,10,11,12,13,14,15,16

       ==> (September 2, 1997 EDT) 9:00,9:20,9:40,10:00,10:20,
                                   ... 16:00,16:20,16:40
           (September 3, 1997 EDT) 9:00,9:20,9:40,10:00,10:20,
                                   ...16:00,16:20,16:40
           ...

    Limit to 3 days to avoid a long test
    */
    let start_date = create!(Date, day: 5, hour: 9, minute: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_interval(20)
        .set_hours(vec![9, 10, 11, 12, 13, 14, 15, 16])
        .set_count(24 * 3)
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, start_date.set_month(12).unwrap());

    assert_eq!(ocurrences.len(), 24 * 3);
    let mut i = 0;
    for hour in 9..17 {
        for minute in vec![0, 20, 40] {
            assert_eq!(
                ocurrences[i],
                create!(Date, day: 5, hour: hour, minute: minute)
            );
            assert_eq!(
                ocurrences[i + 24],
                create!(Date, day: 6, hour: hour, minute: minute)
            );
            assert_eq!(
                ocurrences[i + 48],
                create!(Date, day: 7, hour: hour, minute: minute)
            );
            i += 1;
        }
    }
}

#[test]
fn test_minutely_set_pos_no_exapanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Set pos is 1
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![1])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 5));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 5));
}

#[test]
fn test_minutely_set_pos_2_no_exapanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Set pos is 2
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![2])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 5));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 5));
}

#[test]
fn test_minutely_set_pos_1_expanded_seconds_1() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 45
    Set pos is 1
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![1])
        .set_seconds(vec![45])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 45));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 45));
}

#[test]
fn test_minutely_set_pos_first_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    Set pos is 1
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![1])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 12));
}

#[test]
fn test_minutely_set_pos_second_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![2])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 34));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 34));
}

#[test]
fn test_minutely_set_pos_last_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![-1])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 56));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 56));
}

#[test]
fn test_minutely_set_pos_negative_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![-3])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 12));
}

#[test]
fn test_minutely_set_pos_repeated_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![1, -3])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 2);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, minute: 6, second: 12));
}

#[test]
fn test_minutely_set_pos_invalid_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![5])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 0);
}

#[test]
fn test_minutely_set_pos_multiple_ocurrences_multiple_expanded_seconds() {
    /*
    Starts at 12:05:05 PM and ends at 12:07:00 PM, with a frequency of 1 minute.
    Allowed seconds are 12, 34, 56
    */
    let start_date = create!(Date, minute: 5, second: 5);
    let end_date = create!(Date, minute: 7, second: 0);

    let recurrence = RecurrenceBuilder::new(Frequency::Minutely)
        .set_until_date(end_date.clone())
        .set_positions(vec![1, -1])
        .set_seconds(vec![12, 34, 56])
        .build();

    let ocurrences = recurrence.calculate_ocurrences(start_date, end_date);

    assert_eq!(ocurrences.len(), 4);
    assert_eq!(ocurrences[0], create!(Date, minute: 5, second: 12));
    assert_eq!(ocurrences[1], create!(Date, minute: 5, second: 56));
    assert_eq!(ocurrences[2], create!(Date, minute: 6, second: 12));
    assert_eq!(ocurrences[3], create!(Date, minute: 6, second: 56));
}
