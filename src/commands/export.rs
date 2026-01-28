use crate::models::Task;
use dirs::home_dir;
use std::fs;

pub fn run(project: &str) {
    let base = home_dir().unwrap().join(".pmcli").join(project);
    let tasks: Vec<Task> =
        serde_json::from_str(&fs::read_to_string(base.join("tasks.json")).unwrap()).unwrap();

    let mut csv = "id,status,priority,deadline,description\n".to_string();

    for t in tasks {
        let deadline = t.deadline.map(|d| d.to_string()).unwrap_or("".into());
        csv.push_str(&format!(
            "{},{:?},{},{},\"{}\"\n",
            t.id, t.status, t.priority, deadline, t.description
        ));
    }

    fs::write(base.join("tasks.csv"), csv).unwrap();
    println!("📤 Exported to tasks.csv");
}
