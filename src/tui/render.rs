// src/tui/render.rs
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};
use std::io::{self, stdout};

use super::components;
use super::popup;
use super::state::TuiState;

/// Inicializa la terminal en modo TUI
pub fn init() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restaura la terminal a su estado original
pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Renderiza toda la interfaz según el estado actual
pub fn render(frame: &mut Frame, state: &TuiState) {
    if state.mode == crate::tui::state::UiMode::AddingFolder {
        popup::render_add_folder_popup(frame, state);
    } else {
        render_normal_interface(frame, state);
    }
}

/// Renderiza la interfaz normal (sin popups)
fn render_normal_interface(frame: &mut Frame, state: &TuiState) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    components::render_header(frame, state, main_chunks[0]);
    components::render_book_list(frame, state, body_chunks[0]);
    components::render_book_details(frame, state, body_chunks[1]);
    components::render_footer(frame, state, main_chunks[2]);
}
