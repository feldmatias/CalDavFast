use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct RecurrencePositions {
    positions: Vec<i32>,
}

impl RecurrencePositions {
    pub fn new(positions: Vec<i32>) -> Self {
        Self { positions }
    }

    pub fn apply<T: PartialEq + Copy>(&self, data: Vec<T>) -> Vec<T> {
        if self.positions.is_empty() {
            return data;
        }

        let mut dates_filtered = Vec::new();
        for position in self.positions.iter() {
            let index = self.get_index_for_position(*position, data.len());

            let item = data.get(index);
            if let Some(item) = item {
                if !dates_filtered.contains(item) {
                    dates_filtered.push(*item);
                }
            }
        }

        dates_filtered
    }

    fn get_index_for_position(&self, position: i32, vec_len: usize) -> usize {
        let index = if position >= 0 {
            position - 1
        } else {
            vec_len as i32 + position
        };

        index as usize
    }
}
