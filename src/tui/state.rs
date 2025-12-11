// src/tui/state.rs
// Application state management - the "heart" of the TUI
// This module contains all mutable state that changes as the user interacts with the app

use crate::book::Book;
use std::path::PathBuf;

/// Main state of the terminal interface
/// This struct holds everything the UI needs to render and respond to user actions
pub struct TuiState {
    /// All books currently in the library
    pub books: Vec<Book>,

    /// Index of the currently selected book in the list (0-based)
    pub selected_index: usize,

    /// Whether the user has requested to quit (by pressing 'q')
    pub should_quit: bool,

    /// List of paths that have been scanned (for display in header)
    pub scan_paths: Vec<String>,

    /// Current UI mode (determines what we're rendering and how we handle input)
    pub mode: UiMode,

    /// File browser state (for the "add folder" popup)
    pub browser: FileBrowser,
}

/// Simple file browser for navigating directories
/// Used in the "add folder" modal popup
pub struct FileBrowser {
    /// Current directory we're browsing
    pub current_path: PathBuf,

    /// List of subdirectories in the current directory
    pub entries: Vec<DirEntry>,

    /// Index of the currently selected directory (0-based)
    pub selected_index: usize,
}

/// Represents a single directory entry in the file browser
#[derive(Clone)]
pub struct DirEntry {
    /// Name of the directory (just the folder name, not full path)
    pub name: String,

    /// Full path to the directory
    pub path: PathBuf,

    /// Whether this is a directory (always true in our case, but kept for potential extension)
    pub is_dir: bool,
}

/// Possible UI modes - determines which screen we're showing
/// PartialEq allows comparing modes with ==
/// Clone allows duplicating the enum
/// Copy allows copying by value instead of moving (since it's tiny)
#[derive(PartialEq, Clone, Copy)]
pub enum UiMode {
    /// Normal mode: showing book list and details
    Normal,

    /// Adding folder mode: showing file browser popup
    AddingFolder,
}

/// Actions that the UI can request the main loop to perform
/// These are returned from event handlers when something needs to happen
/// beyond just updating the state
pub enum AppAction {
    /// User selected a folder to add - main loop should scan it for books
    AddFolder(PathBuf),
}

impl FileBrowser {
    /// Creates a new file browser starting at the user's home directory
    ///
    /// # Returns
    /// A FileBrowser with entries loaded from the home directory
    pub fn new() -> Self {
        // Try to get HOME environment variable, default to "/" if it fails
        let home = std::env::var("HOME")
            .ok() // Convert Result to Option
            .map(PathBuf::from) // Convert String to PathBuf
            .unwrap_or_else(|| PathBuf::from("/")); // Default to root if no HOME

        // Create browser with home path
        let mut browser = Self {
            current_path: home,
            entries: Vec::new(),
            selected_index: 0,
        };

        // Load the initial directory entries
        browser.load_entries();

        browser
    }

    /// Loads directory entries from the current path
    /// Filters out hidden files (starting with .) and non-directories
    pub fn load_entries(&mut self) {
        // Clear previous entries
        self.entries.clear();

        // Reset selection to first item
        self.selected_index = 0;

        // Try to read the directory
        if let Ok(read_dir) = std::fs::read_dir(&self.current_path) {
            // Iterate through directory entries
            // flatten() converts Iterator<Result<Entry>> to Iterator<Entry>, skipping errors
            for entry in read_dir.flatten() {
                // Try to get metadata (file type, size, etc.)
                if let Ok(metadata) = entry.metadata() {
                    let path = entry.path();

                    // Get the directory/file name
                    let name = entry
                        .file_name()
                        .to_string_lossy() // Convert OsString to Cow<str>
                        .to_string(); // Convert to owned String

                    // Skip hidden files/directories (starting with .)
                    if name.starts_with('.') {
                        continue;
                    }

                    // Only show directories, not files
                    if metadata.is_dir() {
                        self.entries.push(DirEntry {
                            name,
                            path,
                            is_dir: true,
                        });
                    }
                }
            }
        }

        // Sort entries alphabetically by name
        self.entries.sort_by(|a, b| a.name.cmp(&b.name));
    }

    /// Moves the selection cursor up by one entry
    /// Does nothing if already at the top
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Moves the selection cursor down by one entry
    /// Does nothing if already at the bottom
    pub fn move_down(&mut self) {
        // saturating_sub prevents underflow if entries is empty
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Navigates into the currently selected directory
    /// Reloads entries after changing directory
    pub fn enter_selected(&mut self) {
        // Get the selected entry (if any)
        if let Some(entry) = self.entries.get(self.selected_index) {
            // Only enter if it's a directory
            if entry.is_dir {
                // Update current path to the selected directory
                self.current_path = entry.path.clone();

                // Load entries from the new directory
                self.load_entries();
            }
        }
    }

    /// Navigates up one level in the directory tree (like "cd ..")
    /// Reloads entries after changing directory
    pub fn go_up(&mut self) {
        // Get parent directory (returns Option<&Path>)
        if let Some(parent) = self.current_path.parent() {
            // Convert to owned PathBuf and set as current path
            self.current_path = parent.to_path_buf();

            // Load entries from the parent directory
            self.load_entries();
        }
    }
}

impl TuiState {
    /// Creates a new TUI state with initial data
    ///
    /// # Arguments
    /// * `books` - Initial list of books to display
    /// * `scan_paths` - Paths that were scanned to find these books
    ///
    /// # Returns
    /// A fully initialized TuiState ready to use
    pub fn new(books: Vec<Book>, scan_paths: Vec<String>) -> Self {
        Self {
            books,
            selected_index: 0, // Start with first book selected
            should_quit: false, // Don't quit yet!
            scan_paths,
            mode: UiMode::Normal, // Start in normal mode
            browser: FileBrowser::new(), // Initialize file browser
        }
    }

    /// Gets a reference to the currently selected book (if any)
    ///
    /// # Returns
    /// Some(&Book) if a valid book is selected, None if list is empty or index invalid
    pub fn selected_book(&self) -> Option<&Book> {
        // get() returns Option<&T> - safe indexing that returns None if out of bounds
        self.books.get(self.selected_index)
    }

    /// Moves the book selection cursor up by one
    /// Does nothing if already at the top of the list
    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Moves the book selection cursor down by one
    /// Does nothing if already at the bottom of the list
    pub fn move_selection_down(&mut self) {
        // saturating_sub(1) returns 0 if books is empty, preventing underflow
        if self.selected_index < self.books.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }
}
