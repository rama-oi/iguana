use crate::util::truncate_label;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
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

    let inner_area = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.colors.border))
        .inner(full_area);

    let constraints = vec![Constraint::Length(3), Constraint::Fill(1)];

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    let input_text = app.query.clone();

    let input = Paragraph::new(input_text.as_str())
        .style(Style::default().fg(theme.colors.text))
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
    let name_width = results_area.width.saturating_sub(2);

    let mut items: Vec<ListItem> = app
        .filtered
        .iter()
        .map(|&i| {
            let entry = &app.entries[i];

            ListItem::new(format!(" {}", truncate_label(&entry.name, name_width)))
        })
        .collect();

    items.push(
        ListItem::new(truncate_label(
            &format!("{} :: settings", env!("CARGO_PKG_NAME")),
            name_width,
        ))
        .style(
            Style::default()
                .fg(theme.colors.accent)
                .add_modifier(Modifier::ITALIC),
        ),
    );

    let list = List::new(items).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(list, results_area, &mut app.index_state);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.colors.accent)),
        full_area,
    );
}
