use std::fs;
use std::path::PathBuf;

/// Returns a list of available secrets in the secure folder.
pub fn list_secrets(locker_dir: &PathBuf) -> Vec<String> {
    let mut secrets = Vec::new();
    if let Ok(entries) = fs::read_dir(locker_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    if filename.ends_with(".slock") {
                        secrets.push(filename);
                    }
                }
            }
        }
    }
    secrets
}