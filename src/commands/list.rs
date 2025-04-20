use crate::utils::toolbox::{get_locker_dir, is_this_secret};
use crate::utils::metadata::{read_metadata, has_this_secret_metadata};
use crate::commands::migrate::migrate_metadata;
use crate::MetadataFile;
use crate::SmartLockerError;
use chrono::DateTime;
use colored::Colorize;
use std::fs;
use std::io::{self, Write};

pub fn list_secrets() -> Result<Vec<String>, SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let mut secrets = Vec::new();

    // Charger les métadonnées
    let metadata_result = read_metadata();
    let mut metadata = metadata_result.unwrap_or_else(|_| MetadataFile {
        secrets: Default::default(),
    });

    let mut secrets_to_migrate = Vec::new();

    // Rechercher tous les fichiers .slock
    for entry in fs::read_dir(&locker_dir).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error reading locker directory: {}", e))
    })? {
        let entry = entry.map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error reading directory entry: {}", e))
        })?;
        let path = entry.path();

        // Vérifier si le fichier est un secret valide
        let (is_valid, secret_name) = is_this_secret(&path, true);
        if is_valid {
            if let Some(secret_name) = secret_name {
                // Vérifier les métadonnées
                if !has_this_secret_metadata(&secret_name, &metadata) {
                    secrets_to_migrate.push(secret_name.to_string());
                }
            }
        }
    }

    // Proposer une migration si nécessaire
    if !secrets_to_migrate.is_empty() {
        println!(
            "{}",
            "⚠️ Some metadata are outdated or missing. Do you want to migrate them? (yes/no) : "
                .green()
        );

        // Lire la saisie utilisateur
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "yes" {
            for secret_name in secrets_to_migrate {
                println!(
                    "{}",
                    format!("Migrating metadata for secret '{}'...", secret_name).blue()
                );
                migrate_metadata(Some(&secret_name))?; // Appel à migrate_metadata pour chaque secret
            }
            println!("{}", "✅ Metadata migration completed successfully.".green());
            metadata = read_metadata()?; // Relire les métadonnées après migration
        } else {
            println!("{}", "⚠️ Migration skipped.".yellow());
        }
    }

    // Afficher les secrets
    for (name, secret) in metadata.secrets.iter() {
        let created_at = DateTime::from_timestamp(secret.created_at as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Invalid timestamp".to_string());

        let expire_at = DateTime::from_timestamp(secret.expire_at as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Invalid timestamp".to_string());

        secrets.push(format!(
            "{}\n  Created At: {}  Expire At: {}  Status: {}  Tags: {:?}",
            name.blue(),
            created_at,
            expire_at,
            if secret.expired {
                "Expired".red().to_string()
            } else {
                "Active".green().to_string()
            },
            secret.tags
        ));
    }

    if secrets.is_empty() {
        println!("⚠️ No secrets found.");
    }

    Ok(secrets)
}