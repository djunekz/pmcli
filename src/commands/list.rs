use std::fs;
use dirs::home_dir;
use crate::models::Project;

pub fn run() {
    let base = home_dir().unwrap().join(".pmcli");
    if !base.exists() {
        println!("No projects found");
        return;
    }

    for entry in fs::read_dir(base).unwrap() {
        let dir = entry.unwrap().path();
        let meta_path = dir.join("project.json");

        if meta_path.exists() {
            let meta: Project =
                serde_json::from_str(&fs::read_to_string(meta_path).unwrap()).unwrap();

            println!("📁 {}  (created {})", meta.name, meta.created_at);
        }
    }
}
