use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub total_seconds: i64,
    pub created_at: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourite {
    pub id: i64,
    pub name: String,
    pub duration_seconds: i64,
}
