use serde::{Deserialize, Serialize};

use super::weekday::Weekday;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurrenceVec<T: PartialEq + Clone + Ord> {
    data: Vec<T>,
}

impl<T: PartialEq + Clone + Ord> RecurrenceVec<T> {
    pub fn new(items: Vec<T>) -> RecurrenceVec<T> {
        let mut items = items;
        items.sort();
        RecurrenceVec { data: items }
    }

    pub fn get_or_default(&self, items: Vec<T>) -> RecurrenceVec<T> {
        RecurrenceVec {
            data: if self.data.is_empty() {
                items
            } else {
                self.data.clone()
            },
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.data.iter().any(|x| *x == *item)
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn get_next(&self, item: &T) -> T {
        let index = self.data.iter().position(|x| *x == *item);
        match index {
            Some(i) => {
                if i == self.data.len() - 1 {
                    self.data[0].clone()
                } else {
                    self.data[i + 1].clone()
                }
            }
            None => self.data[0].clone(),
        }
    }
}

impl<T: PartialEq + Clone + Ord> Default for RecurrenceVec<T> {
    fn default() -> Self {
        RecurrenceVec { data: Vec::new() }
    }
}

impl RecurrenceVec<u32> {
    pub fn get_or_default_months(&self) -> RecurrenceVec<u32> {
        self.get_or_default((1..=12).collect())
    }

    pub fn get_or_default_year_days(&self) -> RecurrenceVec<u32> {
        self.get_or_default((1..=366).collect())
    }

    pub fn get_or_default_month_days(&self) -> RecurrenceVec<u32> {
        self.get_or_default((1..=31).collect())
    }

    pub fn get_or_default_hours(&self) -> RecurrenceVec<u32> {
        self.get_or_default((0..=23).collect())
    }

    pub fn get_or_default_minutes(&self) -> RecurrenceVec<u32> {
        self.get_or_default((0..=59).collect())
    }

    pub fn get_or_default_seconds(&self) -> RecurrenceVec<u32> {
        self.get_or_default((0..=59).collect())
    }
}

impl RecurrenceVec<Weekday> {
    pub fn get_or_default_weekdays(&self) -> RecurrenceVec<Weekday> {
        self.get_or_default(vec![
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
            Weekday::Sunday,
        ])
    }
}
