use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Local};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Todo,
    Done,
    Blocked,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub priority: String,
    pub deadline: Option<NaiveDate>,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub default_priority: Option<String>,
    pub date_format: Option<String>,
}
