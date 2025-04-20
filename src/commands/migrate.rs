use crate::utils::metadata::{read_metadata, write_metadata};
use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use crate::{MetadataFile, SecretMetadata};
use chrono::Utc;
use std::fs;

pub fn migrate_metadata(name: Option<&str>) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let mut metadata = read_metadata().unwrap_or_else(|_| MetadataFile {
        secrets: Default::default(),
    });

    let now = Utc::now().timestamp() as u64;

    if let Some(name) = name {
        // Migrer une clé spécifique
        let secret_path = locker_dir.join(format!("{}.slock", name));
        if !secret_path.exists() {
            return Err(SmartLockerError::FileSystemError(format!(
                "Secret file '{}' not found.",
                secret_path.display()
            )));
        }

        // Créer ou mettre à jour les métadonnées pour ce secret
        metadata.secrets.insert(
            name.to_string(),
            SecretMetadata {
                name: name.to_string(),
                created_at: now,
                expire_at: now + (15 * 24 * 60 * 60), // Expiration par défaut : 15 jours
                expired: false,
                tags: Vec::new(),
            },
        );
    } else {
        // Migrer toutes les clés
        for entry in fs::read_dir(&locker_dir).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error reading locker directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                SmartLockerError::FileSystemError(format!("Error reading directory entry: {}", e))
            })?;
            let path = entry.path();

            // Vérifier si le fichier est un secret .slock
            if path.extension().and_then(|ext| ext.to_str()) == Some("slock") {
                let secret_name =
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .ok_or_else(|| {
                            SmartLockerError::FileSystemError(format!(
                                "Invalid secret file name '{}'",
                                path.display()
                            ))
                        })?;

                // Créer ou mettre à jour les métadonnées pour ce secret
                metadata.secrets.insert(
                    secret_name.to_string(),
                    SecretMetadata {
                        name: secret_name.to_string(),
                        created_at: now,
                        expire_at: now + (15 * 24 * 60 * 60), // Expiration par défaut : 15 jours
                        expired: false,
                        tags: Vec::new(),
                    },
                );
            }
        }
    }

    write_metadata(&metadata)?;
    println!("✅ Metadata migration completed successfully.");
    Ok(())
}
