use crate::utils::metadata::{read_metadata, remove_metadata};
use crate::utils::toolbox::get_locker_dir;
use crate::MetadataFile;
use crate::SmartLockerError;
use std::fs;

pub fn remove_secret(name: Option<&str>, remove_all: bool) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;

    if !locker_dir.exists() {
        println!("No secure folder found. Run `init` to create it.");
        return Ok(());
    }

    let mut metadata = read_metadata().unwrap_or_else(|_| MetadataFile {
        secrets: Default::default(),
    });

    if remove_all {
        // Supprimer tous les secrets et leurs fichiers
        for secret_name in metadata.secrets.keys().cloned().collect::<Vec<_>>() {
            let file_path = locker_dir.join(format!("{}.slock", secret_name));
            if file_path.exists() {
                fs::remove_file(&file_path).map_err(|e| {
                    SmartLockerError::FileSystemError(format!("Error when deleting the file: {}", e))
                })?;
            }
        }

        // Supprimer toutes les métadonnées
        remove_metadata(None, &mut metadata)?;
        println!("All secrets and their metadata have been successfully deleted.");
    } else if let Some(secret_name) = name {
        // Vérifier si le fichier existe
        let file_path = locker_dir.join(format!("{}.slock", secret_name));
        if file_path.exists() {
            fs::remove_file(&file_path).map_err(|e| {
                SmartLockerError::FileSystemError(format!("Error when deleting the file: {}", e))
            })?;
            // Supprimer les métadonnées associées
            remove_metadata(Some(secret_name), &mut metadata)?;
            println!("Secret '{}' and its metadata have been successfully deleted.", secret_name);
        } else {
            println!("Secret '{}' doesn't exist.", secret_name);
        }
    } else {
        println!("Please specify a secret name with `-n` or use `--all` to delete all secrets.");
    }

    Ok(())
}