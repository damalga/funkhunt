// src/tui/components.rs
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::state::TuiState;

/// Renderiza la cabecera de la aplicación
pub fn render_header(frame: &mut Frame, state: &TuiState, area: Rect) {
    let path_info = if state.scan_paths.is_empty() {
        "No folders added".to_string()
    } else if state.scan_paths.len() == 1 {
        state.scan_paths[0].clone()
    } else {
        format!("{} folders", state.scan_paths.len())
    };

    let header_text = format!("FunkHunt | Books: {} | {}", state.books.len(), path_info);

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue)),
        );

    frame.render_widget(header, area);
}

/// Renderiza la lista de libros
pub fn render_book_list(frame: &mut Frame, state: &TuiState, area: Rect) {
    let title = format!("Book List ({})", state.books.len());

    let items: Vec<ListItem> = if state.books.is_empty() {
        vec![ListItem::new("No books found. Press 'a' to add a folder.")]
    } else {
        state
            .books
            .iter()
            .enumerate()
            .map(|(i, book)| {
                let style = if i == state.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(book.name.as_str()).style(style)
            })
            .collect()
    };

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

    frame.render_widget(list, area);
}

/// Renderiza los detalles del libro seleccionado
pub fn render_book_details(frame: &mut Frame, state: &TuiState, area: Rect) {
    let details = match state.selected_book() {
        Some(book) => book.get_metadata(),
        None => "Select a book to view details\n\nor press 'a' to add a folder".to_string(),
    };

    let details_widget = Paragraph::new(details)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Book Details"))
        .wrap(Wrap { trim: true });

    frame.render_widget(details_widget, area);
}

/// Renderiza el pie de página normal
pub fn render_footer(frame: &mut Frame, _state: &TuiState, area: Rect) {
    let footer_text = "q: salir | ↑↓: navegar | Enter: abrir libro | a: agregar carpeta";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}

/// Renderiza el pie de página para modo agregar carpeta
pub fn render_adding_folder_footer(frame: &mut Frame, area: Rect) {
    let footer_text = "ENTER: confirmar | ESC: cancelar";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
