use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use chrono::{DateTime, Utc}; // Ajout pour le formatage des dates
use colored::Colorize;
use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct SecretMetadata {
    name: String,
    created_at: u64,
    modified_at: u64,
    tags: Vec<String>,
}

pub fn list_secrets() -> Result<Vec<String>, SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let mut secrets = Vec::new();

    if let Ok(entries) = fs::read_dir(&locker_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    if filename.ends_with(".meta.json") {
                        let metadata_path = locker_dir.join(&filename);
                        let metadata_content = fs::read_to_string(metadata_path).map_err(|e| {
                            SmartLockerError::FileSystemError(format!(
                                "Error reading metadata file: {}",
                                e
                            ))
                        })?;
                        let metadata: SecretMetadata = serde_json::from_str(&metadata_content)
                            .map_err(|e| {
                                SmartLockerError::FileSystemError(format!(
                                    "Error parsing metadata file: {}",
                                    e
                                ))
                            })?;

                        // Convert timestamps to human-readable format
                        let created_at = DateTime::from_timestamp(metadata.created_at as i64, 0)
                            .unwrap_or_else(|| Utc::now())
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string();

                        let modified_at = DateTime::from_timestamp(metadata.modified_at as i64, 0)
                            .unwrap_or_else(|| Utc::now())
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string();

                        secrets.push(format!(
                            "Name: {}\n Created At: {}, Modified At: {}, Tags: {:?}",
                            metadata.name.green(),
                            created_at,
                            modified_at,
                            metadata.tags
                        ));
                    }
                }
            }
        }
    }

    Ok(secrets)
}
