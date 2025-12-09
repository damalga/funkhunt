use crate::book::Book;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_epubs<P: AsRef<Path>>(path: P) -> Vec<Book> {
    let path_ref = path.as_ref();

    if !path_ref.exists() || !path_ref.is_dir() {
        return Vec::new();
    }

    WalkDir::new(path_ref)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("epub"))
                .unwrap_or(false)
        })
        .map(|entry| {
            let name = entry
                .path()
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            Book::new(name, entry.path().to_path_buf())
        })
        .collect()
}
