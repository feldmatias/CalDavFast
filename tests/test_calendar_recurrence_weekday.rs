mod common;
use cal_dav_fast::app::calendar::models::calendar_types::recurrence::weekday::Weekday;
use pretty_assertions::assert_eq;

#[test]
fn test_from_chrono_monday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Mon);
    assert_eq!(weekday, Weekday::Monday);
}

#[test]
fn test_from_chrono_tuesday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Tue);
    assert_eq!(weekday, Weekday::Tuesday);
}

#[test]
fn test_from_chrono_wednesday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Wed);
    assert_eq!(weekday, Weekday::Wednesday);
}

#[test]
fn test_from_chrono_thursday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Thu);
    assert_eq!(weekday, Weekday::Thursday);
}

#[test]
fn test_from_chrono_friday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Fri);
    assert_eq!(weekday, Weekday::Friday);
}

#[test]
fn test_from_chrono_saturday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Sat);
    assert_eq!(weekday, Weekday::Saturday);
}

#[test]
fn test_from_chrono_sunday() {
    let weekday = Weekday::from_chrono(&chrono::Weekday::Sun);
    assert_eq!(weekday, Weekday::Sunday);
}

#[test]
fn test_get_days_diff_to_monday() {
    let from_monday = Weekday::Monday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 7);

    let from_tuesday = Weekday::Monday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 6);

    let from_wednesday = Weekday::Monday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 5);

    let from_thursday = Weekday::Monday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 4);

    let from_friday = Weekday::Monday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 3);

    let from_saturday = Weekday::Monday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 2);

    let from_sunday = Weekday::Monday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 1);
}

#[test]
fn test_get_days_diff_to_tuesday() {
    let from_monday = Weekday::Tuesday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 1);

    let from_tuesday = Weekday::Tuesday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 7);

    let from_wednesday = Weekday::Tuesday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 6);

    let from_thursday = Weekday::Tuesday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 5);

    let from_friday = Weekday::Tuesday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 4);

    let from_saturday = Weekday::Tuesday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 3);

    let from_sunday = Weekday::Tuesday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 2);
}

#[test]
fn test_get_days_diff_to_wednesday() {
    let from_monday = Weekday::Wednesday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 2);

    let from_tuesday = Weekday::Wednesday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 1);

    let from_wednesday = Weekday::Wednesday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 7);

    let from_thursday = Weekday::Wednesday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 6);

    let from_friday = Weekday::Wednesday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 5);

    let from_saturday = Weekday::Wednesday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 4);

    let from_sunday = Weekday::Wednesday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 3);
}

#[test]
fn test_get_days_diff_to_thursday() {
    let from_monday = Weekday::Thursday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 3);

    let from_tuesday = Weekday::Thursday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 2);

    let from_wednesday = Weekday::Thursday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 1);

    let from_thursday = Weekday::Thursday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 7);

    let from_friday = Weekday::Thursday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 6);

    let from_saturday = Weekday::Thursday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 5);

    let from_sunday = Weekday::Thursday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 4);
}

#[test]
fn test_get_days_diff_to_friday() {
    let from_monday = Weekday::Friday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 4);

    let from_tuesday = Weekday::Friday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 3);

    let from_wednesday = Weekday::Friday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 2);

    let from_thursday = Weekday::Friday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 1);

    let from_friday = Weekday::Friday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 7);

    let from_saturday = Weekday::Friday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 6);

    let from_sunday = Weekday::Friday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 5);
}

#[test]
fn test_get_days_diff_to_saturday() {
    let from_monday = Weekday::Saturday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 5);

    let from_tuesday = Weekday::Saturday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 4);

    let from_wednesday = Weekday::Saturday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 3);

    let from_thursday = Weekday::Saturday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 2);

    let from_friday = Weekday::Saturday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 1);

    let from_saturday = Weekday::Saturday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 7);

    let from_sunday = Weekday::Saturday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 6);
}

#[test]
fn test_get_days_diff_to_sunday() {
    let from_monday = Weekday::Sunday.get_days_diff_from(&Weekday::Monday);
    assert_eq!(from_monday, 6);

    let from_tuesday = Weekday::Sunday.get_days_diff_from(&Weekday::Tuesday);
    assert_eq!(from_tuesday, 5);

    let from_wednesday = Weekday::Sunday.get_days_diff_from(&Weekday::Wednesday);
    assert_eq!(from_wednesday, 4);

    let from_thursday = Weekday::Sunday.get_days_diff_from(&Weekday::Thursday);
    assert_eq!(from_thursday, 3);

    let from_friday = Weekday::Sunday.get_days_diff_from(&Weekday::Friday);
    assert_eq!(from_friday, 2);

    let from_saturday = Weekday::Sunday.get_days_diff_from(&Weekday::Saturday);
    assert_eq!(from_saturday, 1);

    let from_sunday = Weekday::Sunday.get_days_diff_from(&Weekday::Sunday);
    assert_eq!(from_sunday, 7);
}
