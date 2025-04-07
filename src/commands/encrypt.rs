use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::Write;

pub fn encrypt(secret: &str, name: &str) -> Result<(), SmartLockerError> {
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
    println!("✅ Secret encrypted and saved in: {:?}", output_path);

    Ok(())
}
