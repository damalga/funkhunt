// src/tui/state.rs
use crate::book::Book;
use std::path::PathBuf;

/// Estado principal de la interfaz de terminal
pub struct TuiState {
    pub books: Vec<Book>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub scan_paths: Vec<String>,
    pub mode: UiMode,
    pub input_buffer: String,
}

/// Modos posibles de la interfaz de usuario
#[derive(PartialEq, Clone, Copy)]
pub enum UiMode {
    Normal,
    AddingFolder,
}

/// Acciones que la interfaz puede solicitar
pub enum AppAction {
    AddFolder(PathBuf),
}

impl TuiState {
    /// Crea un nuevo estado de interfaz
    pub fn new(books: Vec<Book>, scan_paths: Vec<String>) -> Self {
        Self {
            books,
            selected_index: 0,
            should_quit: false,
            scan_paths,
            mode: UiMode::Normal,
            input_buffer: String::new(),
        }
    }

    /// Obtiene el libro actualmente seleccionado
    pub fn selected_book(&self) -> Option<&Book> {
        self.books.get(self.selected_index)
    }

    /// Mueve la selección hacia arriba en la lista
    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Mueve la selección hacia abajo en la lista
    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.books.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }
}
