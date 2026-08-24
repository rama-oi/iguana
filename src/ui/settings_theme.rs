use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub fn draw_settings_themes(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let full_area = frame.area();

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.colors.accent));

    let inner_area = outer_block.inner(full_area);

    let theme_items: Vec<ListItem> = app
        .themes
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let (marker, marker_color) = if i == app.selected_theme {
                ("●", theme.colors.accent)
            } else {
                ("○", theme.colors.text)
            };

            ListItem::new(format!(" {marker} {}", t.name)).style(Style::default().fg(marker_color))
        })
        .collect();

    let themes_list = List::new(theme_items).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(themes_list, inner_area, &mut app.theme_list_state);

    frame.render_widget(outer_block, full_area);
}
