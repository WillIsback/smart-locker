use crate::utils::metadata::{read_metadata, write_metadata};
use crate::utils::config::EncryptionConfig;
use crate::utils::toolbox::get_locker_dir;
use crate::MetadataFile;
use crate::{LockerResult, SecretMetadata, SmartLockerError};
use aes_gcm::aead::Aead;
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn encrypt(
    secret: &str,
    name: &str,
    tags: Vec<String>,
    expiration_days: Option<u64>,
) -> LockerResult<()> {
    let config = EncryptionConfig::new();
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Unable to read symmetric key: {}", e))
    })?;
    let cipher = config.init_cipher(&key_data).map_err(|e| {
        SmartLockerError::EncryptionError(e)
    })?;

    // Générer un nonce aléatoire
    let nonce = config.generate_nonce();

    // Compresser les données
    let mut encoder = config.init_compressor();
    encoder.write_all(secret.as_bytes()).map_err(|e| {
        SmartLockerError::EncryptionError(format!("Error during data compression: {}", e))
    })?;
    let compressed_data = encoder.finish().map_err(|e| {
        SmartLockerError::EncryptionError(format!("Error when finalizing compression: {}", e))
    })?;

    // Chiffrer les données
    let ciphertext = cipher
        .encrypt(&nonce, compressed_data.as_ref())
        .map_err(|e| {
            SmartLockerError::EncryptionError(format!("Error during encryption: {}", e))
        })?;

    // Ajouter une signature versionnée
    let mut output_data = Vec::new();
    output_data.extend_from_slice(config.signature); // Ajouter la signature
    output_data.push(config.format_version); // Ajouter la version
    output_data.extend_from_slice(&nonce); // Ajouter le nonce
    output_data.extend_from_slice(&ciphertext); // Ajouter les données chiffrées

    // Écrire les données chiffrées dans le fichier .slock
    let output_path = locker_dir.join(format!("{}.slock", name));
    fs::write(&output_path, output_data).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error when writing encrypted file: {}", e))
    })?;

    // Charger les métadonnées existantes
    let mut metadata = read_metadata().unwrap_or_else(|_| MetadataFile {
        secrets: Default::default(),
    });

    // Ajouter ou mettre à jour les métadonnées pour ce secret
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiration = now + (expiration_days.unwrap_or(15) * 24 * 60 * 60); // Par défaut : 15 jours
    metadata.secrets.insert(
        name.to_string(),
        SecretMetadata {
            name: name.to_string(),
            created_at: now,
            expire_at: expiration,
            expired: false,
            tags,
        },
    );

    // Sauvegarder les métadonnées mises à jour
    write_metadata(&metadata)?;

    println!(
        "{}",
        format!("✅ Secret '{}' encrypted and stored successfully!", name).green()
    );
    Ok(())
}

