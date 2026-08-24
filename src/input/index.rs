use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::input::command::quit;

pub fn handle_index_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => quit(app),
        _ => {}
    }
}
