pub fn evaluate(query: &str) -> Option<String> {
    let query = query.trim();

    if query.is_empty() {
        return None;
    }

    if !looks_like_math(query) {
        return None;
    }

    match meval::eval_str(query) {
        Ok(value) if value.is_finite() => Some(format_result(value)),
        _ => None,
    }
}

fn looks_like_math(query: &str) -> bool {
    let has_operator = query
        .chars()
        .any(|c| matches!(c, '+' | '-' | '*' | '/' | '^'));

    has_operator
        && query.chars().all(|c| {
            c.is_ascii_digit() || matches!(c, '+' | '-' | '*' | '/' | '.' | '(' | ')' | ' ' | '^')
        })
}

fn format_result(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}

pub fn copy_to_clipboard(text: &str) -> bool {
    try_clipboard_cmd("wl-copy", &[], text)
        || try_clipboard_cmd("xclip", &["-selection", "clipboard"], text)
        || try_clipboard_cmd("xsel", &["--clipboard", "--input"], text)
}

fn try_clipboard_cmd(cmd: &str, args: &[&str], text: &str) -> bool {
    use std::io::Write;
    use std::process::{Command, Stdio};

    if !crate::apps::command_exists(cmd) {
        return false;
    }

    let Ok(mut child) = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        return false;
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }

    child.wait().is_ok()
}
