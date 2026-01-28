use dirs::home_dir;
use std::fs;

use crate::models::{Status, Task};

pub fn run(project: &str, id: u32) {
    let path = home_dir()
        .unwrap()
        .join(".pmcli")
        .join(project)
        .join("tasks.json");

    let mut tasks: Vec<Task> = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

    let mut found = false;

    for task in tasks.iter_mut() {
        if task.id == id {
            task.status = Status::Done;
            found = true;
            break;
        }
    }

    if !found {
        println!("❌ Task with id {} not found", id);
        return;
    }

    fs::write(&path, serde_json::to_string_pretty(&tasks).unwrap()).unwrap();
    println!("✅ Task marked as DONE");
}
