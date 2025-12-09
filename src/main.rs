// src/main.rs
mod book;
mod config;
mod scanner;
mod tui; // Ahora es una carpeta/modulo

use crate::config::{show_usage, Config};
use crate::tui::{handle_key_event, init, render, restore, AppAction, TuiState};
use crossterm::event::{self, Event};

fn main() -> std::io::Result<()> {
    let mut config = Config::from_args();

    if config.show_help {
        show_usage();
        return Ok(());
    }

    let books = config.scan_all_paths();
    let scan_paths: Vec<String> = config
        .scan_paths
        .iter()
        .map(|p| p.display().to_string())
        .collect();

    let mut state = TuiState::new(books, scan_paths);
    let mut terminal = init()?;

    while !state.should_quit {
        terminal.draw(|frame| {
            render(frame, &state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let Some(action) = handle_key_event(key.code, &mut state) {
                    match action {
                        AppAction::AddFolder(path) => {
                            let new_books = scanner::scan_epubs(&path);
                            if !new_books.is_empty() {
                                state.books.extend(new_books);
                                state.scan_paths.push(path.display().to_string());
                                config.scan_paths.push(path);
                            }
                        }
                    }
                }
            }
        }
    }

    restore()?;
    Ok(())
}
