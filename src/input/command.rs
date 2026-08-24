use crate::app::{App, Screen};

pub fn quit(app: &mut App) {
    // if matches!(app.screen, Screen::Themes) {
    //     app.theme_list_state.select(Some(app.selected_theme));
    // }

    app.should_quit = true;
}
