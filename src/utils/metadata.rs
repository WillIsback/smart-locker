use crate::MetadataFile;
use crate::SecretMetadata;
use crate::SmartLockerError;
use crate::utils::toolbox::get_locker_dir;
use std::collections::HashMap;
use std::fs;
use colored::Colorize;

pub fn read_metadata() -> Result<MetadataFile, SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let metadata_path = locker_dir.join("metadata.json");
    if !metadata_path.exists() {
        return Ok(MetadataFile {
            secrets: HashMap::new(),
        });
    }

    let content = fs::read_to_string(&metadata_path).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error reading metadata file: {}", e))
    })?;
    serde_json::from_str(&content).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error parsing metadata file: {}", e))
    })
}

pub fn write_metadata(
    metadata: &MetadataFile,
) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let metadata_path = locker_dir.join("metadata.json");
    let content = serde_json::to_string_pretty(metadata).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error serializing metadata: {}", e))
    })?;
    fs::write(metadata_path, content).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error writing metadata file: {}", e))
    })
}

pub fn has_metadata_file() -> bool {
    let locker_dir = get_locker_dir().unwrap();
    let metadata_path = locker_dir.join("metadata.json");
    metadata_path.exists()
}

pub fn metadata_field_count(secret_metadata: &SecretMetadata) -> usize {
    SecretMetadata::field_count(Some(secret_metadata))
}

pub fn has_this_secret_metadata(secret_name: &str, metadata: &MetadataFile) -> bool {
    if let Some(secret_metadata) = metadata.secrets.get(secret_name) {
        let expected_field_count = SecretMetadata::field_count(None);
        let actual_field_count = SecretMetadata::field_count(Some(secret_metadata));

        if actual_field_count == expected_field_count {
            true // Métadonnées valides
        } else {
            println!(
                "{}",
                format!(
                    "⚠️ Metadata for secret '{}' is outdated. Migration is required.",
                    secret_name
                )
                .yellow()
            );
            false // Métadonnées invalides
        }
    } else {
        println!(
            "{}",
            format!(
                "⚠️ Metadata for secret '{}' is missing. Migration is required.",
                secret_name
            )
            .yellow()
        );
        false // Métadonnées manquantes
    }
}