use dirs::home_dir;
use std::env;
use std::fs;

use crate::models::{Status, Task};

pub fn run(project: &str, filter: Option<&str>) {
    let path = home_dir()
        .unwrap()
        .join(".pmcli")
        .join(project)
        .join("tasks.json");

    let tasks: Vec<Task> = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

    let current_user = env::var("USER").unwrap_or("unknown".into());

    for task in tasks {
        // ===== FILTER =====
        if let Some(f) = filter {
            match f {
                "mine" if task.owner != current_user => continue,
                "todo" if !matches!(task.status, Status::Todo) => continue,
                "done" if !matches!(task.status, Status::Done) => continue,
                "blocked" if !matches!(task.status, Status::Blocked) => continue,
                _ => {}
            }
        }

        let status_label = match task.status {
            Status::Todo => "TODO",
            Status::Done => "DONE",
            Status::Blocked => "BLOCKED",
        };

        println!(
            "[{}] {:<8} | {:<6} | {} | owner: {}",
            task.id, status_label, task.priority, task.description, task.owner
        );
    }
}
