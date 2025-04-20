use crate::utils::metadata::{read_metadata, write_metadata};
use crate::utils::toolbox::get_locker_dir;
use crate::{LockerResult, SecretMetadata, SmartLockerError};
use crate::MetadataFile;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::Write;
use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn encrypt(
    secret: &str,
    name: &str,
    tags: Vec<String>,
    expiration_days: Option<u64>,
) -> LockerResult<()> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Unable to read symmetric key: {}", e))
    })?;
    let key = Key::<Aes256Gcm>::from_slice(&key_data);
    let cipher = Aes256Gcm::new(key);

    // Générer un nonce aléatoire
    let random_bytes = rand::random::<[u8; 12]>();
    let nonce = Nonce::from_slice(&random_bytes);

    // Compresser les données
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(secret.as_bytes()).map_err(|e| {
        SmartLockerError::EncryptionError(format!("Error during data compression: {}", e))
    })?;
    let compressed_data = encoder.finish().map_err(|e| {
        SmartLockerError::EncryptionError(format!("Error when finalizing compression: {}", e))
    })?;

    // Chiffrer les données
    let ciphertext = cipher
        .encrypt(nonce, compressed_data.as_ref())
        .map_err(|e| {
            SmartLockerError::EncryptionError(format!("Error during encryption: {}", e))
        })?;

    // Écrire les données chiffrées dans le fichier .slock
    let output_path = locker_dir.join(format!("{}.slock", name));
    let mut output_data = Vec::new();
    output_data.extend_from_slice(nonce);
    output_data.extend_from_slice(&ciphertext);

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