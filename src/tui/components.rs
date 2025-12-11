// src/tui/components.rs
// Reusable UI components - the building blocks of our interface

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::state::TuiState;

/// Renders the application header showing book count and scanned paths
///
/// The header displays:
/// - App name ("FunkHunt")
/// - Number of books in the library
/// - Scan paths info (single path, multiple paths count, or "No folders added")
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `state` - Current application state
/// * `area` - The rectangular area to draw in
pub fn render_header(frame: &mut Frame, state: &TuiState, area: Rect) {
    // Determine what to show for path info
    let path_info = if state.scan_paths.is_empty() {
        // No folders scanned yet
        "No folders added".to_string()
    } else if state.scan_paths.len() == 1 {
        // Single folder - show its full path
        state.scan_paths[0].clone()
    } else {
        // Multiple folders - just show count
        format!("{} folders", state.scan_paths.len())
    };

    // Build header text: "FunkHunt | Books: 42 | ~/Books"
    let header_text = format!("FunkHunt | Books: {} | {}", state.books.len(), path_info);

    // Create header widget with styling
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan)) // Cyan text
        .block(
            Block::default()
                .borders(Borders::ALL) // Border on all sides
                .style(Style::default().fg(Color::Blue)), // Blue border
        );

    // Draw the widget
    frame.render_widget(header, area);
}

/// Renders the scrollable list of books
///
/// Features:
/// - Shows book filenames
/// - Highlights the currently selected book in yellow/bold
/// - Shows helpful message if list is empty
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `state` - Current application state (books and selection)
/// * `area` - The rectangular area to draw in
pub fn render_book_list(frame: &mut Frame, state: &TuiState, area: Rect) {
    // Title shows total book count
    let title = format!("Book List ({})", state.books.len());

    // Build list items
    let items: Vec<ListItem> = if state.books.is_empty() {
        // Empty list - show helpful message
        vec![ListItem::new("No books found. Press 'a' to add a folder.")]
    } else {
        // Map books to styled list items
        state
            .books
            .iter()
            .enumerate() // Get (index, book) pairs
            .map(|(i, book)| {
                // Style the selected book differently
                let style = if i == state.selected_index {
                    // Selected: yellow and bold
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    // Normal: white
                    Style::default().fg(Color::White)
                };

                // Create list item with book name and style
                ListItem::new(book.name.as_str()).style(style)
            })
            .collect() // Collect into Vec<ListItem>
    };

    // Create List widget with border and title
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

    // Draw the widget
    frame.render_widget(list, area);
}

/// Renders detailed information about the selected book
///
/// Shows:
/// - Book title (filename)
/// - Full path to the EPUB file
/// - File size in KB
///
/// If no book is selected, shows a help message.
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `state` - Current application state
/// * `area` - The rectangular area to draw in
pub fn render_book_details(frame: &mut Frame, state: &TuiState, area: Rect) {
    // Get details text based on whether a book is selected
    let details = match state.selected_book() {
        Some(book) => {
            // Book selected - get its metadata
            book.get_metadata()
        }
        None => {
            // No book selected - show help text
            "Select a book to view details\n\nor press 'a' to add a folder".to_string()
        }
    };

    // Create paragraph widget
    let details_widget = Paragraph::new(details)
        .style(Style::default().fg(Color::White)) // White text
        .block(Block::default().borders(Borders::ALL).title("Book Details")) // Border with title
        .wrap(Wrap { trim: true }); // Wrap long lines, trim whitespace

    // Draw the widget
    frame.render_widget(details_widget, area);
}

/// Renders the footer with keyboard controls help (normal mode)
///
/// Shows available key bindings for the normal mode interface.
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `_state` - Application state (unused, but kept for consistency)
/// * `area` - The rectangular area to draw in
pub fn render_footer(frame: &mut Frame, _state: &TuiState, area: Rect) {
    // Help text showing keyboard controls
    let footer_text = "q: quit | ↑↓: navigate | Enter: open book | a: add folder";

    // Create paragraph widget
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray)) // Gray text
        .block(Block::default().borders(Borders::ALL)); // Border

    // Draw the widget
    frame.render_widget(footer, area);
}

