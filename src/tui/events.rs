// src/tui/events.rs
use crossterm::event::KeyCode;
use std::path::PathBuf;

use super::state::{AppAction, TuiState, UiMode};

/// Procesa eventos de teclado y devuelve acciones a ejecutar
pub fn handle_key_event(key_code: KeyCode, state: &mut TuiState) -> Option<AppAction> {
    match state.mode {
        UiMode::Normal => handle_normal_mode(key_code, state),
        UiMode::AddingFolder => handle_adding_folder_mode(key_code, state),
    }
}

/// Maneja teclas en modo normal
fn handle_normal_mode(key_code: KeyCode, state: &mut TuiState) -> Option<AppAction> {
    match key_code {
        KeyCode::Char('q') => state.should_quit = true,
        KeyCode::Up => state.move_selection_up(),
        KeyCode::Down => state.move_selection_down(),
        KeyCode::Enter => {
            if let Some(book) = state.selected_book() {
                let _ = book.open();
            }
        }
        KeyCode::Char('a') => {
            state.mode = UiMode::AddingFolder;
            state.input_buffer.clear();
        }
        _ => {}
    }
    None
}

/// Maneja teclas en modo agregar carpeta
fn handle_adding_folder_mode(key_code: KeyCode, state: &mut TuiState) -> Option<AppAction> {
    match key_code {
        KeyCode::Enter => {
            let path = PathBuf::from(state.input_buffer.clone());
            state.mode = UiMode::Normal;
            state.input_buffer.clear();

            if path.exists() {
                return Some(AppAction::AddFolder(path));
            }
        }
        KeyCode::Esc => {
            state.mode = UiMode::Normal;
            state.input_buffer.clear();
        }
        KeyCode::Backspace => {
            state.input_buffer.pop();
        }
        KeyCode::Char(c) => {
            state.input_buffer.push(c);
        }
        _ => {}
    }
    None
}
