mod common;
use cal_dav_fast::app::calendar::models::calendar_types::recurrence::recurrence_positions::RecurrencePositions;
use pretty_assertions::assert_eq;

#[test]
fn test_apply_empty() {
    let data = vec![1, 2, 3, 4, 5];
    let positions = RecurrencePositions::new(vec![]);

    let result = positions.apply(data);

    assert_eq!(result, vec!(1, 2, 3, 4, 5));
}

#[test]
fn test_apply_positive() {
    let data = vec![1, 2, 3, 4, 5];
    let positions = RecurrencePositions::new(vec![1, 2, 3]);

    let result = positions.apply(data);

    assert_eq!(result, vec!(1, 2, 3));
}

#[test]
fn test_apply_negative() {
    let data = vec![1, 2, 3, 4, 5];
    let positions = RecurrencePositions::new(vec![-1, -2, -3]);

    let result = positions.apply(data);

    assert_eq!(result, vec!(5, 4, 3));
}

#[test]
fn test_apply_mixed() {
    let data = vec![1, 2, 3, 4, 5];
    let positions = RecurrencePositions::new(vec![1, -1]);

    let result = positions.apply(data);

    assert_eq!(result, vec!(1, 5));
}

#[test]
fn test_apply_mixed_repeated() {
    let data = vec![1, 2, 3, 4, 5];
    let positions = RecurrencePositions::new(vec![1, -5]);

    let result = positions.apply(data);

    assert_eq!(result, vec!(1));
}

#[test]
fn test_apply_one_position() {
    let data = vec![1, 2, 3, 4, 5];
    let positions = RecurrencePositions::new(vec![3]);

    let result = positions.apply(data);

    assert_eq!(result, vec!(3));
}
