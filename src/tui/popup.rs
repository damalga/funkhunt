// src/tui/popup.rs
// File browser modal popup - overlay UI for directory navigation

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use super::state::TuiState;

/// Renders the "add folder" popup over the normal interface
///
/// This creates a full-screen modal overlay effect:
/// 1. Covers the entire screen with a dark background
/// 2. Centers the file browser modal in the middle of the screen
///
/// The modal takes 90% width and 80% height of the full screen,
/// providing a spacious and focused browsing experience.
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `state` - Current application state
pub fn render_add_folder_popup(frame: &mut Frame, state: &TuiState) {
    // Render the modal centered on the entire screen
    // This will:
    // 1. Clear the entire screen
    // 2. Draw a dark background over everything
    // 3. Center the modal dialog (90% width, 80% height)
    render_folder_modal(frame, state, frame.size());
}

/// Renders the actual file browser modal
///
/// The modal features:
/// - Dark background covering the entire screen
/// - Centered dialog box (90% width, 80% height of the screen)
/// - Current path display at the top
/// - Scrollable list of subdirectories
/// - Selected directory is highlighted in yellow/bold
///
/// This function handles the full modal rendering pipeline:
/// 1. Clears the entire area (removes previous frame content)
/// 2. Draws dark background
/// 3. Calculates centered position
/// 4. Renders modal with border and content
///
/// # Arguments
/// * `frame` - The frame buffer to draw on
/// * `state` - Current application state (contains browser state)
/// * `area` - The area where the modal should be drawn (typically full screen)
fn render_folder_modal(frame: &mut Frame, state: &TuiState, area: Rect) {
    // STEP 1: Clear the entire area first
    // This is CRITICAL - it erases whatever was drawn before
    frame.render_widget(Clear, area);

    // STEP 2: Draw dark background over the entire area
    let background = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(background, area);

    // STEP 3: Calculate centered modal rectangle (90% width, 80% height)
    let inner_modal = centered_in_rect(90, 80, area);

    // STEP 4: Clear the modal area (ensures clean background)
    frame.render_widget(Clear, inner_modal);

    // STEP 5: Draw solid background for the modal
    // Using RGB(40,40,40) = dark gray for better contrast
    let modal_bg = Block::default().style(Style::default().bg(Color::Rgb(40, 40, 40)));
    frame.render_widget(modal_bg, inner_modal);

    // STEP 6: Draw border and title for the modal
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" FILE BROWSER ")
        .style(Style::default().bg(Color::Rgb(40, 40, 40)).fg(Color::White));
    frame.render_widget(block, inner_modal);

    // STEP 7: Layout the modal content
    let modal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1) // Leave 1-char margin inside the border
        .constraints([
            Constraint::Length(1),    // Current path display
            Constraint::Min(0),       // Directory list (takes remaining space)
        ])
        .split(inner_modal);

    // STEP 8: Display current path with folder emoji
    let current_path = format!("📁 {}", state.browser.current_path.display());
    let path_display = Paragraph::new(current_path)
        .style(Style::default().fg(Color::Cyan).bg(Color::Rgb(40, 40, 40)));
    frame.render_widget(path_display, modal_chunks[0]);

    // STEP 9: Build the directory list
    let items: Vec<ListItem> = if state.browser.entries.is_empty() {
        // Empty directory - show message
        vec![ListItem::new("(empty)").style(
            Style::default().fg(Color::Gray).bg(Color::Rgb(40, 40, 40))
        )]
    } else {
        // Map directory entries to styled list items
        state
            .browser
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                // Add emoji prefix (folder or file icon)
                let prefix = if entry.is_dir { "📂 " } else { "📄 " };
                let text = format!("{}{}", prefix, entry.name);

                // Style based on selection state
                let style = if i == state.browser.selected_index {
                    // Selected directory: yellow, bold, slightly lighter background
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Rgb(60, 60, 60))
                } else {
                    // Normal directory: white text, standard background
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Rgb(40, 40, 40))
                };

                ListItem::new(text).style(style)
            })
            .collect() // Collect into Vec<ListItem>
    };

    // STEP 10: Create and render the directory list widget
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Directories")
                .style(Style::default().bg(Color::Rgb(40, 40, 40)))
        );
    frame.render_widget(list, modal_chunks[1]);
}

/// Centers a rectangle within another rectangle using percentage sizing
///
/// This helper function is used to create centered modal dialogs.
///
/// # How it works:
/// Given a rectangle to center and percentages for width/height:
/// 1. Calculates margins to center vertically (top, content, bottom)
/// 2. Calculates margins to center horizontally (left, content, right)
/// 3. Returns the centered inner rectangle
///
/// # Example:
/// ```
/// // Create a rectangle that is 90% width and 80% height, centered
/// let modal = centered_in_rect(90, 80, frame.size());
/// ```
///
/// # Arguments
/// * `percent_x` - Percentage of width to use (0-100)
/// * `percent_y` - Percentage of height to use (0-100)
/// * `outer` - The container rectangle to center within
///
/// # Returns
/// A Rect representing the centered area
fn centered_in_rect(percent_x: u16, percent_y: u16, outer: Rect) -> Rect {
    // Calculate vertical layout: top margin, content, bottom margin
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),  // Top margin
            Constraint::Percentage(percent_y),              // Content area
            Constraint::Percentage((100 - percent_y) / 2),  // Bottom margin
        ])
        .split(outer);

    // Calculate horizontal layout: left margin, content, right margin
    // Apply to the middle vertical section (index [1])
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),  // Left margin
            Constraint::Percentage(percent_x),              // Content area
            Constraint::Percentage((100 - percent_x) / 2),  // Right margin
        ])
        .split(vertical[1])[1] // Return only the center rectangle
}
