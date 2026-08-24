use std::io;

use crate::config::{Config, load_config};
use crate::input::index::handle_index_input;
use crate::input::settings::handle_settings_input;
use crate::input::settings_about::handle_settings_about_input;
use crate::input::settings_theme::handle_settings_themes_input;
use crate::theme::{Theme, discover_themes, find_theme_index, install_bundled_themes_if_missing};
use crate::ui::index::draw_index;
use crate::ui::settings::draw_settings;
use crate::ui::settings_about::draw_settings_about;
use crate::ui::settings_theme::draw_settings_themes;

use crossterm::event::{self, Event, KeyEvent};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::ListState};

pub enum Screen {
    Index,
    Settings,
    Themes,
    About,
}

pub struct Entry {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
}
pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub status_message: Option<String>,
    pub last_key_event: Option<KeyEvent>,
    pub themes: Vec<Theme>,
    pub selected_theme: usize,
    pub theme_list_state: ListState,
    pub settings_list_state: ListState,
    pub config: Config,
    pub query: String,
    pub max_len: usize,
    pub status: Option<String>,
    pub entries: Vec<Entry>,
    pub filtered: Vec<usize>,
    pub index_state: ListState,
}

impl App {
    pub fn theme(&self) -> &Theme {
        self.themes
            .get(self.selected_theme)
            .unwrap_or(&self.themes[0])
    }

    pub fn refresh_filter(&mut self) {
        self.filtered = crate::apps::filter_entries(&self.entries, &self.query);
        self.index_state.select(Some(0));
    }

    pub fn launch_app(&self) -> Option<&Entry> {
        let selected = self.index_state.selected()?;
        let entry_idx = *self.filtered.get(selected)?;
        self.entries.get(entry_idx)
    }

    pub fn index_row_count(&self) -> usize {
        self.filtered.len() + 1
    }

    pub fn selected_is_settings(&self) -> bool {
        self.index_state.selected() == Some(self.filtered.len())
    }
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let config = load_config();
    install_bundled_themes_if_missing();
    let themes = discover_themes();
    let selected_theme = find_theme_index(&themes, &config.theme);

    let mut app = App {
        screen: Screen::Index,
        should_quit: false,
        status_message: None,
        last_key_event: None,
        themes,
        selected_theme,
        theme_list_state: ListState::default().with_selected(Some(selected_theme)),
        settings_list_state: ListState::default().with_selected(Some(0)),
        config,
        query: String::new(),
        max_len: 0,
        status: None,
        entries: crate::apps::discover_apps(),
        filtered: Vec::new(),
        index_state: ListState::default(),
    };

    app.refresh_filter();

    loop {
        terminal.draw(|frame| match app.screen {
            Screen::Index => draw_index(frame, &mut app),
            Screen::Settings => draw_settings(frame, &mut app),
            Screen::About => draw_settings_about(frame, &mut app),
            Screen::Themes => draw_settings_themes(frame, &mut app),
        })?;

        if let Event::Key(key) = event::read()? {
            app.last_key_event = Some(key);

            match app.screen {
                Screen::Index => handle_index_input(&mut app, key),
                Screen::Settings => handle_settings_input(&mut app, key),
                Screen::About => handle_settings_about_input(&mut app, key),
                Screen::Themes => handle_settings_themes_input(&mut app, key),
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
