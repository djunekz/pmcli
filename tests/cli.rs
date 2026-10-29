use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct Sandbox {
    home: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let home = std::env::temp_dir().join(format!("pmcli-test-{}-{}", std::process::id(), id));
        fs::create_dir_all(&home).unwrap();
        Sandbox { home }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_pmcli"))
            .args(args)
            .env("HOME", &self.home)
            .env("XDG_CONFIG_HOME", self.home.join(".config"))
            .env("USER", "tester")
            .output()
            .unwrap()
    }

    fn stdout(&self, args: &[&str]) -> String {
        String::from_utf8_lossy(&self.run(args).stdout).into_owned()
    }

    fn project_dir(&self, name: &str) -> PathBuf {
        self.home.join(".pmcli").join(name)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.home).ok();
    }
}

#[test]
fn create_makes_project_files() {
    let sb = Sandbox::new();
    let out = sb.stdout(&["create", "demo"]);
    assert!(out.contains("created"));

    let dir = sb.project_dir("demo");
    assert!(dir.join("project.json").is_file());
    assert!(dir.join("tasks.json").is_file());
    assert!(dir.join("notes.md").is_file());
    assert_eq!(fs::read_to_string(dir.join("tasks.json")).unwrap(), "[]");
}

#[test]
fn create_rejects_duplicate_project() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    let out = sb.stdout(&["create", "demo"]);
    assert!(out.contains("already exists"));
}

#[test]
fn list_reports_no_projects_on_fresh_home() {
    let sb = Sandbox::new();
    assert!(sb.stdout(&["list"]).contains("No projects found"));
}

#[test]
fn list_shows_created_projects() {
    let sb = Sandbox::new();
    sb.run(&["create", "alpha"]);
    sb.run(&["create", "beta"]);
    let out = sb.stdout(&["list"]);
    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn add_task_uses_default_priority() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["add-task", "demo", "Write docs"]);

    let raw = fs::read_to_string(sb.project_dir("demo").join("tasks.json")).unwrap();
    let tasks: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(tasks[0]["id"], 1);
    assert_eq!(tasks[0]["description"], "Write docs");
    assert_eq!(tasks[0]["priority"], "medium");
    assert_eq!(tasks[0]["status"], "Todo");
    assert_eq!(tasks[0]["owner"], "tester");
}

#[test]
fn add_task_stores_priority_and_deadline() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&[
        "add-task",
        "demo",
        "Ship release",
        "--priority",
        "high",
        "--deadline",
        "2026-02-01",
    ]);

    let raw = fs::read_to_string(sb.project_dir("demo").join("tasks.json")).unwrap();
    let tasks: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(tasks[0]["priority"], "high");
    assert_eq!(tasks[0]["deadline"], "2026-02-01");
}

#[test]
fn add_task_assigns_incrementing_ids() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["add-task", "demo", "First"]);
    sb.run(&["add-task", "demo", "Second"]);

    let out = sb.stdout(&["tasks", "demo"]);
    assert!(out.contains("[1]"));
    assert!(out.contains("[2]"));
}

#[test]
fn done_task_marks_task_as_done() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["add-task", "demo", "First"]);
    let out = sb.stdout(&["done-task", "demo", "1"]);
    assert!(out.contains("DONE"));

    let listing = sb.stdout(&["tasks", "demo"]);
    assert!(listing.contains("DONE"));
}

#[test]
fn done_task_reports_unknown_id() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    let out = sb.stdout(&["done-task", "demo", "42"]);
    assert!(out.contains("not found"));
}

#[test]
fn tasks_filter_separates_todo_and_done() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["add-task", "demo", "Open item"]);
    sb.run(&["add-task", "demo", "Closed item"]);
    sb.run(&["done-task", "demo", "2"]);

    let todo = sb.stdout(&["tasks", "demo", "--filter", "todo"]);
    assert!(todo.contains("Open item"));
    assert!(!todo.contains("Closed item"));

    let done = sb.stdout(&["tasks", "demo", "--filter", "done"]);
    assert!(done.contains("Closed item"));
    assert!(!done.contains("Open item"));
}

#[test]
fn tasks_filter_mine_matches_current_user() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["add-task", "demo", "Mine"]);
    let out = sb.stdout(&["tasks", "demo", "--filter", "mine"]);
    assert!(out.contains("Mine"));
}

#[test]
fn note_appends_markdown_bullet() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["note", "demo", "first note"]);
    sb.run(&["note", "demo", "second note"]);

    let notes = fs::read_to_string(sb.project_dir("demo").join("notes.md")).unwrap();
    assert_eq!(notes, "- first note\n- second note\n");
}

#[test]
fn export_writes_csv_with_header_and_rows() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&[
        "add-task",
        "demo",
        "Export me",
        "--priority",
        "low",
        "--deadline",
        "2026-03-15",
    ]);
    sb.run(&["export", "demo"]);

    let csv = fs::read_to_string(sb.project_dir("demo").join("tasks.csv")).unwrap();
    let mut lines = csv.lines();
    assert_eq!(
        lines.next().unwrap(),
        "id,status,priority,deadline,description"
    );
    assert_eq!(lines.next().unwrap(), "1,Todo,low,2026-03-15,\"Export me\"");
}

#[test]
fn custom_config_changes_date_format() {
    let sb = Sandbox::new();
    let config_dir = sb.home.join(".config").join("pmcli");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(
        config_dir.join("config.toml"),
        "default_priority = \"high\"\ndate_format = \"%d/%m/%Y\"\n",
    )
    .unwrap();

    sb.run(&["create", "demo"]);
    sb.run(&["add-task", "demo", "Localized", "--deadline", "01/02/2026"]);

    let raw = fs::read_to_string(sb.project_dir("demo").join("tasks.json")).unwrap();
    let tasks: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(tasks[0]["priority"], "high");
    assert_eq!(tasks[0]["deadline"], "2026-02-01");
}

#[test]
fn version_flag_prints_package_version() {
    let sb = Sandbox::new();
    let out = sb.stdout(&["--version"]);
    assert!(out.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_subcommand_fails() {
    let sb = Sandbox::new();
    assert!(!sb.run(&["does-not-exist"]).status.success());
}
