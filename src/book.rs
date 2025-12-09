use std::path::PathBuf;

#[derive(Clone)]
pub struct Book {
    pub name: String,
    pub path: PathBuf,
}

impl Book {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    pub fn get_metadata(&self) -> String {
        match std::fs::metadata(&self.path) {
            Ok(meta) => {
                let size_kb = meta.len() / 1024;
                format!(
                    "Title: {}\n\nPath: {}\n\nSize: {} KB",
                    self.name,
                    self.path.display(),
                    size_kb
                )
            }
            Err(_) => "Error reading metadata".to_string(),
        }
    }

    pub fn open(&self) -> std::io::Result<()> {
        #[cfg(target_os = "linux")]
        let command = "xdg-open";

        #[cfg(target_os = "macos")]
        let command = "open";

        #[cfg(target_os = "windows")]
        let command = "cmd";

        #[cfg(target_os = "windows")]
        let args = ["/C", "start", self.path.to_str().unwrap_or("")];

        #[cfg(not(target_os = "windows"))]
        let args = [self.path.to_str().unwrap_or("")];

        let mut cmd = std::process::Command::new(command);

        #[cfg(target_os = "windows")]
        cmd.args(&args);

        #[cfg(not(target_os = "windows"))]
        cmd.arg(args[0]);

        cmd.spawn()?;
        Ok(())
    }
}
