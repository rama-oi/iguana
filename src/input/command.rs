use crate::app::App;

pub fn quit(app: &mut App) {
    app.should_quit = true;
}
