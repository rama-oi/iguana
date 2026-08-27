use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::input::command::handle_shortcut;
use crate::input::command::quit;

pub fn handle_index_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char(c) = key.code {
            handle_shortcut(app, c.to_ascii_lowercase());
        }
    } else {
        match key.code {
            KeyCode::Esc => quit(app),

            KeyCode::Char(c) => {
                app.status = None;

                if app.query.len() < app.max_len {
                    app.query.push(c);
                    app.refresh_filter();
                }
            }

            KeyCode::Backspace => {
                app.status = None;
                app.query.pop();
                app.refresh_filter();
            }

            KeyCode::Enter => {
                app.status = None;

                if let Some(result) = app.calculator_result.clone() {
                    if crate::input::calculator::copy_to_clipboard(&result) {
                        app.should_quit = true;
                    } else {
                        app.status = Some("Could not copy result to clipboard".to_string());
                    }
                } else if let Some(entry) = app.launch_app() {
                    let exec = entry.exec.clone();
                    let terminal = entry.terminal;

                    crate::apps::launch(&exec, terminal);
                    app.should_quit = true;
                }
            }

            KeyCode::Down => {
                let row_count = app.index_row_count();
                app.status = None;

                let selected = app.index_state.selected().unwrap_or(0);

                let next = if selected >= row_count - 1 {
                    0
                } else {
                    selected + 1
                };

                app.index_state.select(Some(next));
            }

            KeyCode::Up => {
                let row_count = app.index_row_count();
                app.status = None;

                let selected = app.index_state.selected().unwrap_or(0);

                let prev = if selected == 0 {
                    row_count - 1
                } else {
                    selected - 1
                };

                app.index_state.select(Some(prev));
            }

            _ => {}
        }
    }
}
