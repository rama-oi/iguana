use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandEntry {
    pub label: String,
    pub cmd: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default, rename = "command")]
    pub commands: Vec<CommandEntry>,
}

fn default_theme() -> String {
    "catppuccin-mocha".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            commands: Vec::new(),
        }
    }
}

const SAMPLE_CONFIG: &str = include_str!("../coqui_config.toml.sample");

fn config_path() -> Option<std::path::PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(std::path::PathBuf::from(home).join(".config/rama/coqui_config.toml"))
}

pub fn load_config() -> Config {
    let Some(path) = config_path() else {
        return Config::default();
    };

    if !path.exists() {
        if let Some(parent) = path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                eprintln!("Failed to create {}: {err}", parent.display());
            }
        }

        if let Err(err) = std::fs::write(&path, SAMPLE_CONFIG) {
            eprintln!("Failed to write {}: {err}", path.display());
        }
    }

    match std::fs::read_to_string(&path) {
        Ok(raw) => match toml::from_str(&raw) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Failed to parse {}: {err}", path.display());
                Config::default()
            }
        },
        Err(err) => {
            eprintln!("Failed to read {}: {err}", path.display());
            Config::default()
        }
    }
}

fn save_config_key(key: &str, value: &str) {
    let Some(path) = config_path() else {
        return;
    };

    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let new_line = format!("{key} = \"{value}\"");

    let mut found = false;
    let mut lines: Vec<String> = raw
        .lines()
        .map(|line| {
            if !found && line_key(line) == Some(key) {
                found = true;
                new_line.clone()
            } else {
                line.to_string()
            }
        })
        .collect();

    if !found {
        lines.push(new_line);
    }

    let updated = lines.join("\n") + "\n";

    if let Err(err) = std::fs::write(&path, updated) {
        eprintln!("Failed to write {}: {err}", path.display());
    }
}

fn line_key(line: &str) -> Option<&str> {
    let (key, _) = line.split_once('=')?;
    Some(key.trim())
}

pub fn save_theme(theme_slug: &str) {
    save_config_key("theme", theme_slug);
}
