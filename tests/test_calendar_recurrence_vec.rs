use cal_dav_fast::app::calendar::models::calendar_types::recurrence::{
    recurrence_vec::RecurrenceVec, weekday::Weekday,
};
use pretty_assertions::assert_eq;

#[test]
fn test_get_or_default_with_data() {
    let recurrence_vec = RecurrenceVec::new(vec![1, 2, 3]);
    let response = recurrence_vec.get_or_default(vec![4, 5]);
    assert_eq!(response.length(), 3);
    assert!(response.contains(&1));
    assert!(response.contains(&2));
    assert!(response.contains(&3));
}

#[test]
fn test_get_or_default_without_data() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default(vec![4, 5, 6]);
    assert_eq!(response.length(), 3);
    assert!(response.contains(&4));
    assert!(response.contains(&5));
    assert!(response.contains(&6));
}

#[test]
fn test_get_next_when_item_does_not_exist_returns_first_element_greater_than_item() {
    let recurrence_vec = RecurrenceVec::new(vec![1, 2, 18]);
    let response = recurrence_vec.get_next(&4);
    assert_eq!(response, 18);
}

#[test]
fn test_get_next_when_item_does_not_exist_and_no_item_greater_than_item_returns_first_element() {
    let recurrence_vec = RecurrenceVec::new(vec![1, 2, 3]);
    let response = recurrence_vec.get_next(&4);
    assert_eq!(response, 1);
}

#[test]
fn test_get_next_when_item_does_not_exist_and_all_items_are_greater() {
    let recurrence_vec = RecurrenceVec::new(vec![5, 18]);
    let response = recurrence_vec.get_next(&3);
    assert_eq!(response, 5);
}

#[test]
fn test_get_next_when_item_is_last_element_returns_first_element() {
    let recurrence_vec = RecurrenceVec::new(vec![1, 2, 3]);
    let response = recurrence_vec.get_next(&3);
    assert_eq!(response, 1);
}

#[test]
fn test_get_next_when_item_is_not_last_element_returns_next_element() {
    let recurrence_vec = RecurrenceVec::new(vec![1, 2, 3]);
    let response = recurrence_vec.get_next(&2);
    assert_eq!(response, 3);
}

#[test]
fn test_get_next_when_item_is_first_element_returns_second_element() {
    let recurrence_vec = RecurrenceVec::new(vec![1, 2, 3]);
    let response = recurrence_vec.get_next(&1);
    assert_eq!(response, 2);
}

#[test]
fn test_get_next_when_item_exists_and_is_the_only_element() {
    let recurrence_vec = RecurrenceVec::new(vec![1]);
    let response = recurrence_vec.get_next(&1);
    assert_eq!(response, 1);
}

#[test]
fn test_get_next_when_item_does_not_exist_and_there_is_only_one_element() {
    let recurrence_vec = RecurrenceVec::new(vec![1]);
    let response = recurrence_vec.get_next(&6);
    assert_eq!(response, 1);
}

#[test]
fn test_default_months() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_months();
    assert_eq!(response.length(), 12);
    assert!(response.contains(&1));
    assert!(response.contains(&5));
    assert!(response.contains(&12));
}

#[test]
fn test_default_year_days() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_year_days();
    assert_eq!(response.length(), 366);
    assert!(response.contains(&1));
    assert!(response.contains(&100));
    assert!(response.contains(&366));
}

#[test]
fn test_default_month_days() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_month_days();
    assert_eq!(response.length(), 31);
    assert!(response.contains(&1));
    assert!(response.contains(&15));
    assert!(response.contains(&31));
}

#[test]
fn test_default_hours() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_hours();
    assert_eq!(response.length(), 24);
    assert!(response.contains(&0));
    assert!(response.contains(&12));
    assert!(response.contains(&23));
}

#[test]
fn test_default_minutes() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_minutes();
    assert_eq!(response.length(), 60);
    assert!(response.contains(&0));
    assert!(response.contains(&30));
    assert!(response.contains(&59));
}

#[test]
fn test_default_seconds() {
    let recurrence_vec = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_seconds();
    assert_eq!(response.length(), 60);
    assert!(response.contains(&0));
    assert!(response.contains(&30));
    assert!(response.contains(&59));
}

#[test]
fn test_default_week_days() {
    let recurrence_vec: RecurrenceVec<Weekday> = RecurrenceVec::default();
    let response = recurrence_vec.get_or_default_weekdays();
    assert_eq!(response.length(), 7);
    assert!(response.contains(&Weekday::Monday));
    assert!(response.contains(&Weekday::Tuesday));
    assert!(response.contains(&Weekday::Wednesday));
    assert!(response.contains(&Weekday::Thursday));
    assert!(response.contains(&Weekday::Friday));
    assert!(response.contains(&Weekday::Saturday));
    assert!(response.contains(&Weekday::Sunday));
}

#[test]
fn test_new_orders_items() {
    let recurrence_vec = RecurrenceVec::new(vec![3, 1, 4, 2]);
    assert_eq!(recurrence_vec.length(), 4);
    assert_eq!(recurrence_vec.get_next(&1), 2);
    assert_eq!(recurrence_vec.get_next(&2), 3);
    assert_eq!(recurrence_vec.get_next(&3), 4);
    assert_eq!(recurrence_vec.get_next(&4), 1);
}
