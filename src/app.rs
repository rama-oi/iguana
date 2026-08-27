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

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
}
pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub calculator_result: Option<String>,
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
    pub system_entries: Vec<Entry>,
    pub custom_entries: Vec<Entry>,
    pub using_custom: bool,
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
        self.calculator_result = crate::input::calculator::evaluate(&self.query);

        self.filtered = if self.calculator_result.is_some() {
            Vec::new()
        } else {
            crate::apps::filter_entries(&self.entries, &self.query)
        };

        self.index_state.select(Some(0));
    }

    pub fn launch_app(&self) -> Option<&Entry> {
        let selected = self.index_state.selected()?;
        let entry_idx = *self.filtered.get(selected)?;
        self.entries.get(entry_idx)
    }

    pub fn index_row_count(&self) -> usize {
        if self.calculator_result.is_some() {
            1
        } else {
            self.filtered.len() + 1
        }
    }

    pub fn toggle_command_source(&mut self) {
        if self.custom_entries.is_empty() {
            return;
        }

        self.using_custom = !self.using_custom;
        self.entries = if self.using_custom {
            self.custom_entries.clone()
        } else {
            self.system_entries.clone()
        };

        self.query.clear();
        self.status = None;
        self.refresh_filter();
    }
}

fn custom_entries_from_config(config: &Config) -> Vec<Entry> {
    config
        .commands
        .iter()
        .map(|command| Entry {
            name: command.label.clone(),
            exec: command.cmd.clone(),
            terminal: false,
        })
        .collect()
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let config = load_config();
    install_bundled_themes_if_missing();
    let themes = discover_themes();
    let selected_theme = find_theme_index(&themes, &config.theme);

    let system_entries = crate::apps::discover_apps();
    let custom_entries = custom_entries_from_config(&config);
    let using_custom = !custom_entries.is_empty();
    let entries = if using_custom {
        custom_entries.clone()
    } else {
        system_entries.clone()
    };

    let mut app = App {
        screen: Screen::Index,
        should_quit: false,
        calculator_result: None,
        last_key_event: None,
        themes,
        selected_theme,
        theme_list_state: ListState::default().with_selected(Some(selected_theme)),
        settings_list_state: ListState::default().with_selected(Some(0)),
        config,
        query: String::new(),
        max_len: 0,
        status: None,
        entries,
        system_entries,
        custom_entries,
        using_custom,
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
