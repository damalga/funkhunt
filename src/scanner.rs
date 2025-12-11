// src/scanner.rs
// Recursively scans directories for EPUB files

use crate::book::Book;
use std::path::Path;
use walkdir::WalkDir; // External crate for recursive directory traversal

/// Scans a directory (recursively) for EPUB files and returns them as Book objects
///
/// # Generic Parameters
/// * `P: AsRef<Path>` - Accepts any type that can be converted to a Path reference
///   This allows passing String, &str, PathBuf, &Path, etc.
///
/// # Arguments
/// * `path` - The directory path to scan
///
/// # Returns
/// A Vec<Book> containing all found EPUB files, or empty Vec if none found
pub fn scan_epubs<P: AsRef<Path>>(path: P) -> Vec<Book> {
    // Convert the generic path type to a Path reference
    let path_ref = path.as_ref();

    // Validate the path exists and is a directory
    // Return empty vector if invalid
    if !path_ref.exists() || !path_ref.is_dir() {
        return Vec::new();
    }

    // Use the iterator pattern to process directory entries
    // This is a functional programming approach: transform data through a pipeline
    WalkDir::new(path_ref) // Start recursive directory walker
        .into_iter() // Convert to iterator
        // Filter out errors, keep only Ok entries
        // filter_map combines filter + map in one step
        .filter_map(|entry| entry.ok())
        // Keep only entries with .epub extension
        .filter(|entry| {
            entry
                .path() // Get the file path
                .extension() // Get file extension (returns Option<&OsStr>)
                .and_then(|ext| ext.to_str()) // Convert OsStr to &str (may fail)
                .map(|ext| ext.eq_ignore_ascii_case("epub")) // Check if extension is "epub"
                .unwrap_or(false) // If any step failed (no extension, can't convert), return false
        })
        // Transform each entry into a Book struct
        .map(|entry| {
            // Extract the filename to use as book name
            let name = entry
                .path() // Get the full path
                .file_name() // Get just the filename (returns Option<&OsStr>)
                .and_then(|n| n.to_str()) // Convert to &str
                .unwrap_or("Unknown") // Default to "Unknown" if conversion fails
                .to_string(); // Convert &str to owned String

            // Create a new Book with the extracted name and full path
            Book::new(name, entry.path().to_path_buf())
        })
        .collect() // Collect all Book objects into a Vec<Book>
}
