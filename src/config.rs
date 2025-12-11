// src/config.rs
// Configuration and command-line argument parsing

use std::path::PathBuf;

/// Application configuration parsed from command-line arguments
/// #[derive(Debug)] allows pretty-printing the struct for debugging
#[derive(Debug)]
pub struct Config {
    /// List of paths to scan for EPUB files
    pub scan_paths: Vec<PathBuf>,

    /// Whether user requested help (--help or -h)
    pub show_help: bool,
}

impl Config {
    /// Parses command-line arguments and returns a Config struct
    ///
    /// # Examples of valid command lines:
    /// - `funkhunt` - No paths, starts with empty library
    /// - `funkhunt ~/Books` - Scans ~/Books for EPUBs
    /// - `funkhunt ~/Books ~/Documents/EPUBs` - Scans multiple paths
    /// - `funkhunt -h` or `funkhunt --help` - Shows help and exits
    ///
    /// # Returns
    /// A Config struct with parsed arguments
    pub fn from_args() -> Self {
        // Get all command-line arguments except the first one (which is the program name)
        // std::env::args() returns an iterator of Strings
        let args: Vec<String> = std::env::args().skip(1).collect();

        // If no arguments provided, return config with empty paths
        if args.is_empty() {
            Self {
                scan_paths: Vec::new(),
                show_help: false,
            }
        }
        // If user requested help, set show_help flag
        else if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            Self {
                scan_paths: Vec::new(),
                show_help: true,
            }
        }
        // Otherwise, treat all arguments as paths to scan
        else {
            // Convert each String argument into a PathBuf
            let scan_paths: Vec<PathBuf> = args.into_iter().map(PathBuf::from).collect();

            Self {
                scan_paths,
                show_help: false,
            }
        }
    }

    /// Scans all configured paths and returns all found books
    ///
    /// # Returns
    /// A Vec<Book> containing all EPUB files found in all scan_paths
    pub fn scan_all_paths(&self) -> Vec<crate::book::Book> {
        use crate::scanner::scan_epubs;

        // Accumulator for all books across all paths
        let mut all_books = Vec::new();

        // Scan each path and add results to accumulator
        for path in &self.scan_paths {
            let mut books = scan_epubs(path);
            // append() moves all elements from books into all_books
            all_books.append(&mut books);
        }

        all_books
    }
}

/// Prints usage information to stdout
///
/// Shows the user how to use the application, including:
/// - Available command-line options
/// - Example usage
/// - In-app keyboard controls
pub fn show_usage() {
    println!("FunkHunt - P2P Book Sharing");
    println!("=============================\n");

    // Command-line usage
    println!("Usage: funkhunt [PATH...]");
    println!("       funkhunt -h | --help\n");

    // Usage examples
    println!("Examples:");
    println!("  funkhunt                    # Start with empty library");
    println!("  funkhunt ~/Books            # Start with specific folder");
    println!("  funkhunt -h                 # Show this help\n");

    // Keyboard controls inside the app
    println!("In-app controls:");
    println!("  a          : Add folder from within the app");
    println!("  ↑/↓        : Navigate book list");
    println!("  Enter      : Open selected book");
    println!("  q          : Quit application");
}
