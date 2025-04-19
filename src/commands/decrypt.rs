use crate::utils::toolbox::get_locker_dir;
use crate::LockerResult;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;

pub fn decrypt(name: &str) -> LockerResult<String> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to read the symmetric key".to_string())
    })?;
    let key = Key::<Aes256Gcm>::from_slice(&key_data);

    // Initialiser AES-GCM avec la clé
    let cipher = Aes256Gcm::new(key);

    // Lire le fichier chiffré
    let input_path = locker_dir.join(format!("{}.slock", name));
    let encrypted_data = fs::read(&input_path).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to read the encrypted file".to_string())
    })?;

    // Extraire le nonce et les données chiffrées
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    // Déchiffrer les données
    let decrypted_data = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| SmartLockerError::DecryptionError("Decryption failed".to_string()))?;

    // Décompresser les données
    let mut decoder = GzDecoder::new(&decrypted_data[..]);
    let mut decompressed_data = String::new();
    decoder
        .read_to_string(&mut decompressed_data)
        .map_err(|_| {
            SmartLockerError::FileSystemError("Failed to decompress the data".to_string())
        })?;

    Ok(decompressed_data)
}
