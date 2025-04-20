use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct SecretMetadata {
    name: String,
    created_at: u64,
    modified_at: u64,
    tags: Vec<String>,
}

pub fn encrypt(secret: &str, name: &str, tags: Vec<String>) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");

    let key_data = fs::read(&key_path).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Unable to read symmetric key: {}", e))
    })?;
    let key = Key::<Aes256Gcm>::from_slice(&key_data);
    let cipher = Aes256Gcm::new(key);
    let random_bytes = rand::random::<[u8; 12]>();
    let nonce = Nonce::from_slice(&random_bytes);

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(secret.as_bytes()).map_err(|e| {
        SmartLockerError::EncryptionError(format!("Error during data compression: {}", e))
    })?;
    let compressed_data = encoder.finish().map_err(|e| {
        SmartLockerError::EncryptionError(format!("Error when finalizing compression: {}", e))
    })?;

    let ciphertext = cipher
        .encrypt(nonce, compressed_data.as_ref())
        .map_err(|e| {
            SmartLockerError::EncryptionError(format!("Error during encryption: {}", e))
        })?;

    let output_path = locker_dir.join(format!("{}.slock", name));
    let mut output_data = Vec::new();
    output_data.extend_from_slice(nonce);
    output_data.extend_from_slice(&ciphertext);

    fs::write(&output_path, output_data).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error when writing encrypted file: {}", e))
    })?;

    // Save metadata
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let metadata = SecretMetadata {
        name: name.to_string(),
        created_at: now,
        modified_at: now,
        tags,
    };
    let metadata_path = locker_dir.join(format!("{}.meta.json", name));
    let metadata_json = serde_json::to_string(&metadata).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error serializing metadata: {}", e))
    })?;
    fs::write(&metadata_path, metadata_json).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Error writing metadata file: {}", e))
    })?;

    println!("✅ Secret encrypted and saved in: {:?}", output_path);

    Ok(())
}
