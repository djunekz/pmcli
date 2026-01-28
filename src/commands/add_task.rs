use chrono::NaiveDate;
use dirs::home_dir;
use std::env;
use std::fs;

use crate::config;
use crate::models::{Status, Task};

pub fn run(project: &str, desc: &str, priority: Option<&str>, deadline: Option<&str>) {
    let path = home_dir()
        .unwrap()
        .join(".pmcli")
        .join(project)
        .join("tasks.json");

    let mut tasks: Vec<Task> = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

    let cfg = config::load();

    let date_format = cfg.date_format.as_deref().unwrap_or("%Y-%m-%d");

    let parsed_deadline = deadline.map(|d| {
        NaiveDate::parse_from_str(d, date_format).expect("Invalid date format (check config)")
    });

    let prio = priority
        .map(|p| p.to_string())
        .or(cfg.default_priority)
        .unwrap_or_else(|| "medium".to_string());

    let owner = env::var("USER").unwrap_or_else(|_| "unknown".to_string());

    let id = tasks.len() as u32 + 1;

    tasks.push(Task {
        id,
        description: desc.to_string(),
        status: Status::Todo,
        priority: prio,
        deadline: parsed_deadline,
        owner,
    });

    fs::write(&path, serde_json::to_string_pretty(&tasks).unwrap()).unwrap();

    println!("➕ Task added");
}
