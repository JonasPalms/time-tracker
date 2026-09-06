use serde::{Deserialize, Serialize};

pub use time_tracker_core::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourite {
    pub id: i64,
    pub name: String,
    pub duration_seconds: i64,
}
