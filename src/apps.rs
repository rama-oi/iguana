use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::app::Entry;

pub fn discover_apps() -> Vec<Entry> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut entries: Vec<Entry> = Vec::new();

    for dir in application_dirs() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };

        for item in read_dir.filter_map(Result::ok) {
            let path = item.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("desktop") {
                continue;
            }

            let Some(entry) = parse_desktop_file(&path) else {
                continue;
            };

            if seen.insert(entry.name.to_lowercase()) {
                entries.push(entry);
            }
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}

fn application_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(home) = std::env::var_os("HOME") {
        let home = PathBuf::from(home);
        dirs.push(home.join(".local/share/applications"));
        dirs.push(home.join(".local/share/flatpak/exports/share/applications"));
    }

    if let Some(data_dirs) = std::env::var_os("XDG_DATA_DIRS") {
        for dir in std::env::split_paths(&data_dirs) {
            dirs.push(dir.join("applications"));
        }
    } else {
        dirs.push(PathBuf::from("/usr/local/share/applications"));
        dirs.push(PathBuf::from("/usr/share/applications"));
    }

    dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));

    dirs
}

fn parse_desktop_file(path: &Path) -> Option<Entry> {
    let content = std::fs::read_to_string(path).ok()?;

    let mut in_entry_section = false;
    let mut name: Option<String> = None;
    let mut exec: Option<String> = None;
    let mut entry_type: Option<String> = None;
    let mut no_display = false;
    let mut hidden = false;
    let mut terminal = false;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') {
            in_entry_section = line == "[Desktop Entry]";
            continue;
        }

        if !in_entry_section {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        match key.trim() {
            "Name" => name = name.or_else(|| Some(value.trim().to_string())),
            "Exec" => exec = Some(value.trim().to_string()),
            "Type" => entry_type = Some(value.trim().to_string()),
            "NoDisplay" => no_display = value.trim().eq_ignore_ascii_case("true"),
            "Hidden" => hidden = value.trim().eq_ignore_ascii_case("true"),
            "Terminal" => terminal = value.trim().eq_ignore_ascii_case("true"),
            _ => {}
        }
    }

    if entry_type.as_deref().unwrap_or("Application") != "Application" {
        return None;
    }

    if no_display || hidden {
        return None;
    }

    let name = name?;
    let exec = clean_exec(&exec?);

    if exec.is_empty() {
        return None;
    }

    Some(Entry {
        name,
        exec,
        terminal,
    })
}

fn clean_exec(raw: &str) -> String {
    raw.split_whitespace()
        .filter(|token| {
            !matches!(
                *token,
                "%f" | "%F" | "%u" | "%U" | "%i" | "%c" | "%k" | "%d" | "%D" | "%n" | "%N"
                    | "%v" | "%m"
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn filter_entries(entries: &[Entry], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..entries.len()).collect();
    }

    let query = query.to_lowercase();

    let mut scored: Vec<(usize, usize)> = entries
        .iter()
        .enumerate()
        .filter_map(|(i, entry)| entry.name.to_lowercase().find(query.as_str()).map(|pos| (i, pos)))
        .collect();

    scored.sort_by(|a, b| {
        a.1.cmp(&b.1)
            .then_with(|| entries[a.0].name.to_lowercase().cmp(&entries[b.0].name.to_lowercase()))
    });

    scored.into_iter().map(|(i, _)| i).collect()
}

pub fn launch(exec: &str, terminal: bool) {
    let command_line = if terminal {
        wrap_in_terminal(exec)
    } else {
        exec.to_string()
    };

    let detached = format!("setsid -f {command_line} < /dev/null > /dev/null 2>&1");

    let _ = Command::new("sh")
        .arg("-c")
        .arg(detached)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

fn wrap_in_terminal(exec: &str) -> String {
    let term = std::env::var("TERMINAL")
        .ok()
        .filter(|t| !t.is_empty())
        .or_else(|| {
            ["alacritty", "kitty", "foot", "wezterm", "gnome-terminal", "konsole", "xterm"]
                .iter()
                .find(|candidate| command_exists(candidate))
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "xterm".to_string());

    format!("{term} -e sh -c '{exec}'")
}

fn command_exists(cmd: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(cmd).is_file()))
        .unwrap_or(false)
}

