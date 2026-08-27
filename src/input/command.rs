use crate::app::App;
use crate::router;

pub fn quit(app: &mut App) {
    app.should_quit = true;
}

pub fn handle_shortcut(app: &mut App, c: char) {
    match c {
        // 'l' => '',
        's' => router::go_to_settings(app),
        _ => {}
    }
}
