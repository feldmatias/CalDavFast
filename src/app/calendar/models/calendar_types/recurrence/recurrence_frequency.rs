use serde::{Deserialize, Serialize};

pub mod recurrence_minutely;
pub mod recurrence_secondly;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}
