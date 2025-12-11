// src/book.rs
// Data model for an EPUB book and methods to interact with it

use std::path::PathBuf;

/// Represents a single EPUB book in our collection
/// #[derive(Clone)] allows us to create copies of Book structs when needed
#[derive(Clone)]
pub struct Book {
    /// Display name of the book (usually the filename)
    pub name: String,

    /// Full filesystem path to the EPUB file
    /// PathBuf is like String but for file paths, handles OS differences automatically
    pub path: PathBuf,
}

impl Book {
    /// Creates a new Book instance
    ///
    /// # Arguments
    /// * `name` - The display name for the book
    /// * `path` - The full path to the EPUB file
    ///
    /// # Returns
    /// A new Book struct
    pub fn new(name: String, path: PathBuf) -> Self {
        // Self { name, path } is shorthand for Self { name: name, path: path }
        Self { name, path }
    }

    /// Gets metadata about the book for display in the UI
    ///
    /// # Returns
    /// A formatted string with title, path, and file size
    pub fn get_metadata(&self) -> String {
        // Try to read file metadata (size, permissions, etc.)
        match std::fs::metadata(&self.path) {
            Ok(meta) => {
                // Convert file size from bytes to kilobytes
                let size_kb = meta.len() / 1024;

                // Format a nice display string with multiple lines
                format!(
                    "Title: {}\n\nPath: {}\n\nSize: {} KB",
                    self.name,
                    self.path.display(), // .display() formats path correctly for current OS
                    size_kb
                )
            }
            // If reading metadata fails, return an error message
            Err(_) => "Error reading metadata".to_string(),
        }
    }

    /// Opens the book using the system's default EPUB viewer
    ///
    /// Uses different commands depending on the operating system:
    /// - Linux: xdg-open
    /// - macOS: open
    /// - Windows: cmd /C start
    ///
    /// # Returns
    /// Ok(()) if the command was spawned successfully, Err otherwise
    pub fn open(&self) -> std::io::Result<()> {
        // Conditional compilation: these #[cfg] attributes make code compile only on specific OS

        // On Linux, use xdg-open to open with default application
        #[cfg(target_os = "linux")]
        let command = "xdg-open";

        // On macOS, use open command
        #[cfg(target_os = "macos")]
        let command = "open";

        // On Windows, use cmd.exe
        #[cfg(target_os = "windows")]
        let command = "cmd";

        // Windows needs special arguments: /C start <file>
        #[cfg(target_os = "windows")]
        let args = ["/C", "start", self.path.to_str().unwrap_or("")];

        // Linux and macOS just need the file path as argument
        #[cfg(not(target_os = "windows"))]
        let args = [self.path.to_str().unwrap_or("")];

        // Create the command
        let mut cmd = std::process::Command::new(command);

        // Add arguments based on OS
        #[cfg(target_os = "windows")]
        cmd.args(&args);

        #[cfg(not(target_os = "windows"))]
        cmd.arg(args[0]);

        // Spawn the process (run it in background, don't wait for it to finish)
        // The ? operator returns early if spawn() fails
        cmd.spawn()?;

        // Return success
        Ok(())
    }
}
