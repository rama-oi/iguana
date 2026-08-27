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

    let outer_block = Block::default().padding(Padding::proportional(1));

    let inner_area = outer_block.inner(full_area);

    let constraints = vec![
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ];

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

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
    let name_width = results_area.width.saturating_sub(2);

    let items: Vec<ListItem> = if let Some(result) = &app.calculator_result {
        vec![
            ListItem::new(format!(
                " = {}",
                truncate_label(result, name_width.saturating_sub(2))
            ))
            .style(
                Style::default()
                    .fg(theme.colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    } else {
        let items: Vec<ListItem> = app
            .filtered
            .iter()
            .map(|&i| {
                let entry = &app.entries[i];
                ListItem::new(format!(" {}", truncate_label(&entry.name, name_width)))
            })
            .collect();

        items
    };

    let list = List::new(items).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(list, results_area, &mut app.index_state);

    frame.render_widget(outer_block, full_area);

    let separator = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::new().fg(theme.colors.border));

    frame.render_widget(separator, vertical[2]);

    let help = Paragraph::new(" [↑↓]  [^s]  [^l]").style(Style::default().fg(theme.colors.accent));

    frame.render_widget(help, vertical[3]);
}
