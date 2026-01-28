use std::fs;
use std::path::PathBuf;

use crate::models::Config;

pub fn load() -> Config {
    let path = config_path();

    if let Ok(data) = fs::read_to_string(&path) {
        toml::from_str(&data).unwrap_or(default())
    } else {
        default()
    }
}

fn default() -> Config {
    Config {
        default_priority: Some("medium".into()),
        date_format: Some("%Y-%m-%d".into()),
    }
}

fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("pmcli");
    path.push("config.toml");
    path
}
