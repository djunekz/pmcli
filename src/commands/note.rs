use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run(project: &str, note: &str) {
    let path = home_dir()
        .unwrap()
        .join(".pmcli")
        .join(project)
        .join("notes.md");

    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    writeln!(file, "- {}", note).unwrap();
    println!("📝 Note added");
}
