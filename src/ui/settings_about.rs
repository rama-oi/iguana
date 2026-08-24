use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::Paragraph,
};

use crate::app::App;

pub fn draw_settings_about(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(full_area);

    let content = Paragraph::new(format!(
        "{} {}\n{}\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(theme.colors.text));

    frame.render_widget(content, vertical[1]);
}
