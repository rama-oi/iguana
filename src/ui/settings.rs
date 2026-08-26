use ratatui::{
    Frame,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Padding},
};

use crate::app::App;

pub const ITEM_SWITCH_THEME: usize = 0;
pub const ITEM_ABOUT: usize = 1;
pub const ITEM_COUNT: usize = 2;

pub fn draw_settings(frame: &mut Frame, app: &mut App) {
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

    let items = [
        format!(" Switch Theme\n {}", theme.name),
        " About".to_string(),
    ];

    let settings_list = List::new(items.into_iter().map(ListItem::new)).highlight_style(
        Style::default()
            .fg(theme.colors.selection_fg)
            .bg(theme.colors.selection_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(settings_list, inner_area, &mut app.settings_list_state);

    frame.render_widget(outer_block, full_area);
}
