// src/main.rs
// Entry point of the FunkHunt application - a TUI for managing EPUB book collections

// Module declarations - these tell Rust about the other files in our project
mod book;      // Book data model
mod config;    // CLI argument parsing
mod scanner;   // EPUB file scanning
mod tui;       // Terminal User Interface components

// Import items from our modules that we'll use in main()
use crate::config::{show_usage, Config};
use crate::tui::{handle_key_event, init, render, restore, AppAction, TuiState};
use crossterm::event::{self, Event};

/// Main function - the entry point of the application
/// Returns Result<(), std::io::Error> because terminal operations can fail
fn main() -> std::io::Result<()> {
    // Parse command-line arguments into a Config struct
    let mut config = Config::from_args();

    // If user passed --help or -h, show usage and exit early
    if config.show_help {
        show_usage();
        return Ok(()); // Ok(()) means success with no value
    }

    // Scan all provided paths for EPUB files
    let books = config.scan_all_paths();

    // Convert PathBuf objects to String for display in the UI
    // .iter() creates an iterator, .map() transforms each element, .collect() gathers results
    let scan_paths: Vec<String> = config
        .scan_paths
        .iter()
        .map(|p| p.display().to_string())
        .collect();

    // Initialize application state with found books and scanned paths
    let mut state = TuiState::new(books, scan_paths);

    // Initialize terminal in TUI mode (raw mode + alternate screen)
    // The ? operator propagates errors up if init() fails
    let mut terminal = init()?;

    // Main event loop - runs until user quits (presses 'q')
    while !state.should_quit {
        // Draw the interface
        // terminal.draw() takes a closure that receives a Frame to draw on
        terminal.draw(|frame| {
            render(frame, &state); // &state = immutable borrow, we only read state here
        })?;

        // Poll for keyboard events with a 100ms timeout
        // This prevents the loop from blocking forever and allows periodic redraws
        if event::poll(std::time::Duration::from_millis(100))? {
            // Check if the event is a keyboard event
            if let Event::Key(key) = event::read()? {
                // Process the key press and get back an optional action
                // &mut state = mutable borrow, handle_key_event can modify state
                if let Some(action) = handle_key_event(key, &mut state) {
                    // If an action was returned, execute it
                    match action {
                        // User selected a folder to load
                        AppAction::AddFolder(path) => {
                            // Scan the selected path for EPUB files
                            let new_books = scanner::scan_epubs(&path);

                            // Only update if we found at least one book
                            if !new_books.is_empty() {
                                // REPLACE books collection (not add to it)
                                state.books = new_books;

                                // REPLACE scan paths with just the new one
                                state.scan_paths = vec![path.display().to_string()];

                                // REPLACE config paths
                                config.scan_paths = vec![path.clone()];

                                // Reset selection to first book
                                state.selected_index = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    // Restore terminal to normal mode (disable raw mode, leave alternate screen)
    restore()?;

    // Return success
    Ok(())
}
