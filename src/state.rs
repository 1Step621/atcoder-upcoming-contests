use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Contest {
    pub start_time: DateTime<Utc>,
    pub name: String,
    pub duration: u32, // in minutes
    pub rated_range: String,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub contests: Arc<Mutex<Vec<Contest>>>,
}
