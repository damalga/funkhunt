use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub scan_paths: Vec<PathBuf>,
    pub show_help: bool,
}

impl Config {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();

        if args.is_empty() {
            Self {
                scan_paths: Vec::new(),
                show_help: false,
            }
        } else if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            Self {
                scan_paths: Vec::new(),
                show_help: true,
            }
        } else {
            let scan_paths: Vec<PathBuf> = args.into_iter().map(PathBuf::from).collect();

            Self {
                scan_paths,
                show_help: false,
            }
        }
    }

    pub fn scan_all_paths(&self) -> Vec<crate::book::Book> {
        use crate::scanner::scan_epubs;

        let mut all_books = Vec::new();

        for path in &self.scan_paths {
            let mut books = scan_epubs(path);
            all_books.append(&mut books);
        }

        all_books
    }
}

pub fn show_usage() {
    println!("FunkHunt - P2P Book Sharing");
    println!("=============================\n");
    println!("Usage: funkbook [PATH...]");
    println!("       funkbook -h | --help\n");
    println!("Examples:");
    println!("  funkbook                    # Start with empty library");
    println!("  funkbook ~/Books            # Start with specific folder");
    println!("  funkbook -h                 # Show this help\n");
    println!("In-app controls:");
    println!("  a          : Add folder from within the app");
    println!("  ↑/↓        : Navigate book list");
    println!("  Enter      : Open selected book");
    println!("  q          : Quit application");
}
