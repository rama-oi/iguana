pub fn truncate_label(label: &str, max_width: u16) -> String {
    let max_width = max_width as usize;

    if max_width == 0 {
        return String::new();
    }

    if label.chars().count() <= max_width {
        return label.to_string();
    }

    if max_width == 1 {
        return "…".to_string();
    }

    let truncated: String = label.chars().take(max_width - 1).collect();
    format!("{truncated}…")
}
