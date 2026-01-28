use std::fs;
use dirs::home_dir;
use chrono::Local;

use crate::models::Project;

pub fn run(name: &str) {
    let base = home_dir().unwrap().join(".pmcli");
    fs::create_dir_all(&base).ok();

    let project = base.join(name);
    if project.exists() {
        println!("❌ Project already exists");
        return;
    }

    fs::create_dir_all(&project).unwrap();

    let meta = Project {
        name: name.to_string(),
        created_at: Local::now(), // ✅ FIXED
    };

    fs::write(
        project.join("project.json"),
        serde_json::to_string_pretty(&meta).unwrap(),
    )
    .unwrap();

    fs::write(project.join("tasks.json"), "[]").unwrap();
    fs::write(project.join("notes.md"), "").unwrap();

    println!("✅ Project '{}' created", name);
}
