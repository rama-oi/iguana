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
    let has_operator = query.chars().any(|c| matches!(c, '+' | '-' | '*' | '/'));

    has_operator
        && query.chars().all(|c| {
            c.is_ascii_digit() || matches!(c, '+' | '-' | '*' | '/' | '.' | '(' | ')' | ' ')
        })
}

fn format_result(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}
