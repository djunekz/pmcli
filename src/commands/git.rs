use std::process::Command;
use std::env;
use std::path::PathBuf;
use dirs::home_dir;
use git2::{Repository, Signature};

fn project_path(project: &str) -> PathBuf {
    home_dir().unwrap()
        .join(".pmcli")
        .join(project)
}

pub fn init(project: &str) {
    let path = project_path(project);
    Repository::init(&path).expect("Failed to init git repo");
    println!("📁 Git repository initialized");
}

pub fn commit(project: &str, message: &str) {
    let path = project_path(project);
    let repo = Repository::open(&path).expect("Not a git repository");

    let mut index = repo.index().unwrap();
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let sig = repo.signature()
        .unwrap_or(Signature::now("pmcli", "pmcli@local").unwrap());

    let parent = repo.head()
        .ok()
        .and_then(|h| h.target())
        .and_then(|t| repo.find_commit(t).ok());

    match parent {
        Some(p) => {
            repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&p]).unwrap();
        }
        None => {
            repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[]).unwrap();
        }
    }

    println!("✅ Commit created");
}

pub fn push(project: &str) {
    let path = project_path(project);

    // 1️⃣ NORMAL PUSH (USE GIT CREDENTIAL)
    let normal = Command::new("git")
        .arg("push")
        .current_dir(&path)
        .status()
        .expect("Failed to run git push");

    if normal.success() {
        println!("⬆️  Push successful (git credential)");
        return;
    }

    // 2️⃣ FALLBACK TOKEN
    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            println!("❌ Push failed and no GITHUB_TOKEN set");
            return;
        }
    };

    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .current_dir(&path)
        .output()
        .expect("Failed to read git remote");

    let remote = String::from_utf8(output.stdout).unwrap();
    let remote = remote.trim();

    if !remote.starts_with("https://") {
        println!("❌ Remote is not HTTPS (cannot inject token)");
        return;
    }

    let auth_remote = remote.replacen(
        "https://",
        &format!("https://{}@", token),
        1,
    );

    let status = Command::new("git")
        .arg("push")
        .arg(&auth_remote)
        .current_dir(&path)
        .status()
        .expect("Failed to run git push (token)");

    if status.success() {
        println!("⬆️  Push successful (token fallback)");
    } else {
        println!("❌ Push failed even with token");
    }
}

pub fn pull(project: &str) {
    let path = project_path(project);

    // 1️⃣ NORMAL PULL
    let normal = Command::new("git")
        .arg("pull")
        .current_dir(&path)
        .status()
        .expect("Failed to run git pull");

    if normal.success() {
        println!("⬇️  Pull successful (git credential)");
        return;
    }

    // 2️⃣ FALLBACK TOKEN
    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            println!("❌ Pull failed and no GITHUB_TOKEN set");
            return;
        }
    };

    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .current_dir(&path)
        .output()
        .expect("Failed to read git remote");

    let remote = String::from_utf8(output.stdout).unwrap();
    let remote = remote.trim();

    if !remote.starts_with("https://") {
        println!("❌ Remote is not HTTPS (cannot inject token)");
        return;
    }

    let auth_remote = remote.replacen(
        "https://",
        &format!("https://{}@", token),
        1,
    );

    let status = Command::new("git")
        .arg("pull")
        .arg(&auth_remote)
        .current_dir(&path)
        .status()
        .expect("Failed to run git pull (token)");

    if status.success() {
        println!("⬇️  Pull successful (token fallback)");
    } else {
        println!("❌ Pull failed even with token");
    }
}
