use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;

pub fn decrypt(name: &str) -> Result<String, SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Unable to read symmetric key: {}", e))
    })?;
    let key = Key::<Aes256Gcm>::from_slice(&key_data);

    // Initialiser AES-GCM avec la clé
    let cipher = Aes256Gcm::new(key);

    // Lire le fichier chiffré
    let input_path = locker_dir.join(format!("{}.slock", name));
    let encrypted_data = fs::read(&input_path).map_err(|e| {
        SmartLockerError::FileSystemError(format!("Unable to read encrypted file: {}", e))
    })?;

    // Extraire le nonce et les données chiffrées
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    // Déchiffrer les données
    let decrypted_data = cipher.decrypt(nonce, ciphertext).map_err(|e| {
        SmartLockerError::DecryptionError(format!("Error during decryption: {}", e))
    })?;

    // Décompresser les données
    let mut decoder = GzDecoder::new(&decrypted_data[..]);
    let mut decompressed_data = String::new();
    decoder
        .read_to_string(&mut decompressed_data)
        .map_err(|e| {
            SmartLockerError::DecryptionError(format!("Error during data decompression: {}", e))
        })?;

    Ok(decompressed_data)
}
