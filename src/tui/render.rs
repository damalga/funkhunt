// src/tui/render.rs
// Terminal initialization, restoration, and top-level rendering

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

/// Initializes the terminal in TUI mode
///
/// This function:
/// 1. Enables "raw mode" - terminal captures each keypress immediately without waiting for Enter
/// 2. Enters the "alternate screen" - saves the current terminal content and uses a fresh buffer
///
/// # Returns
/// A Terminal object that we can use to draw frames, or an error if initialization fails
pub fn init() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    // Enable raw mode - keys are processed immediately, not line-buffered
    enable_raw_mode()?;

    // Enter alternate screen - like vim, we use a separate buffer
    // When we exit, the user's original terminal content will be restored
    stdout().execute(EnterAlternateScreen)?;

    // Create and return a Terminal with crossterm backend using stdout
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restores the terminal to its original state
///
/// This function:
/// 1. Disables raw mode - returns terminal to normal line-buffered mode
/// 2. Leaves alternate screen - restores the user's original terminal content
///
/// Should always be called before exiting the application!
pub fn restore() -> io::Result<()> {
    // Disable raw mode - return to normal terminal behavior
    disable_raw_mode()?;

    // Leave alternate screen - show the original terminal content again
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

/// Main render function - draws the appropriate interface based on current mode
///
/// # Arguments
/// * `frame` - The frame buffer to draw on (provided by ratatui's terminal.draw())
/// * `state` - Current application state (determines what we render)
pub fn render(frame: &mut Frame, state: &TuiState) {
    // Check current mode and render accordingly
    if state.mode == crate::tui::state::UiMode::AddingFolder {
        // Show file browser popup over the normal interface
        popup::render_add_folder_popup(frame, state);
    } else {
        // Show normal book list interface
        render_normal_interface(frame, state);
    }
}

/// Renders the normal interface (book list + details, no popup)
///
/// Layout structure:
/// ```
/// ┌─────────────────┐
/// │    Header (3)   │  <- Book count, scanned paths
/// ├─────────┬───────┤
/// │ Book    │ Book  │  <- Main body (flexible height)
/// │ List    │Details│
/// │ (50%)   │ (50%) │
/// ├─────────┴───────┤
/// │   Footer (3)    │  <- Keyboard controls help
/// └─────────────────┘
/// ```
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `state` - Current application state
fn render_normal_interface(frame: &mut Frame, state: &TuiState) {
    // Divide screen vertically into 3 sections
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Header: fixed 3 lines
            Constraint::Min(0),       // Body: takes all remaining space
            Constraint::Length(3),    // Footer: fixed 3 lines
        ])
        .split(frame.size()); // Split the entire frame

    // Divide the body horizontally into 2 columns
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left column: book list (50%)
            Constraint::Percentage(50), // Right column: book details (50%)
        ])
        .split(main_chunks[1]); // Split the middle section (body)

    // Render each component into its designated area
    components::render_header(frame, state, main_chunks[0]);
    components::render_book_list(frame, state, body_chunks[0]);
    components::render_book_details(frame, state, body_chunks[1]);
    components::render_footer(frame, state, main_chunks[2]);
}
