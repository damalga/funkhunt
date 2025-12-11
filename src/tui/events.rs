// src/tui/events.rs
// Keyboard event handling - translates keypresses into state changes and actions

use crossterm::event::{KeyCode, KeyEvent};

use super::state::{AppAction, TuiState, UiMode};

/// Main event handler - dispatches to mode-specific handlers
///
/// This is the entry point for all keyboard events. It looks at the current
/// UI mode and delegates to the appropriate handler function.
///
/// # Arguments
/// * `key_event` - The keyboard event from crossterm
/// * `state` - Mutable reference to application state (we can modify it)
///
/// # Returns
/// * `None` - Event was handled entirely within state
/// * `Some(AppAction)` - Event requires main loop to perform an action
pub fn handle_key_event(key_event: KeyEvent, state: &mut TuiState) -> Option<AppAction> {
    // Dispatch based on current mode
    match state.mode {
        UiMode::Normal => handle_normal_mode(key_event, state),
        UiMode::AddingFolder => handle_adding_folder_mode(key_event, state),
    }
}

/// Handles keyboard events in Normal mode (book list view)
///
/// # Key bindings:
/// * `q` - Quit the application
/// * `↑` - Move selection up in book list
/// * `↓` - Move selection down in book list
/// * `Enter` - Open the selected book with system viewer
/// * `a` - Switch to AddingFolder mode (file browser popup)
///
/// # Arguments
/// * `key_event` - The keyboard event
/// * `state` - Mutable reference to application state
///
/// # Returns
/// Always returns None (no actions needed, everything is handled in state)
fn handle_normal_mode(key_event: KeyEvent, state: &mut TuiState) -> Option<AppAction> {
    // Pattern match on the key that was pressed
    match key_event.code {
        // 'q' key quits the application
        KeyCode::Char('q') => state.should_quit = true,

        // Arrow keys navigate the book list
        KeyCode::Up => state.move_selection_up(),
        KeyCode::Down => state.move_selection_down(),

        // Enter opens the selected book
        KeyCode::Enter => {
            // Get the selected book (if any)
            if let Some(book) = state.selected_book() {
                // Try to open it, ignore errors (using _ = discards the Result)
                let _ = book.open();
            }
        }

        // 'a' key opens the file browser to add a folder
        KeyCode::Char('a') => {
            // Switch to AddingFolder mode
            state.mode = UiMode::AddingFolder;

            // Reload browser entries (refreshes the current directory)
            state.browser.load_entries();
        }

        // Any other key is ignored
        _ => {}
    }

    // No actions needed - everything was handled in state
    None
}

/// Handles keyboard events in AddingFolder mode (file browser popup)
///
/// # Key bindings:
/// * `↑` - Move selection up in directory list
/// * `↓` - Move selection down in directory list
/// * `→` or `l` - Enter the selected directory
/// * `←` or `h` - Go up one directory level (like "cd ..")
/// * `Enter` - Confirm selection (add current directory to library)
/// * `Esc` - Cancel and return to Normal mode
///
/// # Arguments
/// * `key_event` - The keyboard event
/// * `state` - Mutable reference to application state
///
/// # Returns
/// * `None` - Event was handled in state
/// * `Some(AppAction::AddFolder)` - User confirmed a folder selection
fn handle_adding_folder_mode(key_event: KeyEvent, state: &mut TuiState) -> Option<AppAction> {
    match key_event.code {
        // Arrow keys navigate the directory list
        KeyCode::Up => {
            state.browser.move_up();
        }
        KeyCode::Down => {
            state.browser.move_down();
        }

        // Enter confirms selection - add the current directory
        KeyCode::Enter => {
            // Clone the current path (we'll return it as an action)
            let path = state.browser.current_path.clone();

            // Return to Normal mode
            state.mode = UiMode::Normal;

            // Return action for main loop to handle
            return Some(AppAction::AddFolder(path));
        }

        // Right arrow or 'l' - navigate into selected directory (vim-style)
        KeyCode::Right | KeyCode::Char('l') => {
            state.browser.enter_selected();
        }

        // Left arrow or 'h' - navigate up one level (vim-style)
        KeyCode::Left | KeyCode::Char('h') => {
            state.browser.go_up();
        }

        // Esc cancels - return to Normal mode without adding anything
        KeyCode::Esc => {
            state.mode = UiMode::Normal;
        }

        // Any other key is ignored
        _ => {}
    }

    // No action needed (unless we returned early with Enter)
    None
}
