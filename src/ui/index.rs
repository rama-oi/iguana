use crate::util::truncate_label;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table},
};

use crate::app::App;

pub fn draw_index(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    frame.render_widget(
        Block::default().style(
            Style::default()
                .bg(theme.colors.background)
                .fg(theme.colors.text),
        ),
        full_area,
    );

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Fill(1)])
        .split(full_area);

    let input_text = app.query.clone();
    let input_style = Style::default().fg(theme.colors.text);
    let input = Paragraph::new(input_text.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1))
                .border_style(Style::default().fg(theme.colors.border)),
        );

    let query_area = vertical[0];
    app.max_len = query_area.width.saturating_sub(4) as usize;

    frame.set_cursor_position((
        query_area.x + 2 + input_text.chars().count() as u16,
        query_area.y + 1,
    ));

    frame.render_widget(input, query_area);

    let results_area = vertical[1];

    if app.filtered.is_empty() {
        let message = if app.entries.is_empty() {
            "No applications found"
        } else {
            "No matches"
        };

        let empty = Paragraph::new(message).style(Style::default().fg(theme.colors.border));
        frame.render_widget(empty, results_area);
        return;
    }

    let name_width = results_area.width.saturating_sub(2);
    let rows = app.filtered.iter().map(|&i| {
        let entry = &app.entries[i];
        Row::new(vec![Cell::from(truncate_label(&entry.name, name_width))])
    });

    let table = Table::new(rows, [Constraint::Fill(1)])
        .row_highlight_style(
            Style::default()
                .fg(theme.colors.selection_fg)
                .bg(theme.colors.selection_bg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(table, results_area, &mut app.index_state);
}
