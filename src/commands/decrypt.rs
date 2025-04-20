use crate::utils::toolbox::get_locker_dir;
use crate::utils::metadata::{read_metadata, write_metadata};
use crate::LockerResult;
use crate::SecretMetadata;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn decrypt(name: &str) -> LockerResult<String> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");
    let metadata_path = locker_dir.join(format!("{}.meta.json", name));

    // Lire les métadonnées
    let metadata_content = fs::read_to_string(&metadata_path).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to read metadata file".to_string())
    })?;
    let metadata: SecretMetadata = serde_json::from_str(&metadata_content).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to parse metadata file".to_string())
    })?;

    // Vérifier l'expiration
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if now > metadata.expire_at {
        // Marquer la clé comme expirée
        let mut updated_metadata = metadata.clone();
        updated_metadata.expired = true;

        // Sauvegarder les métadonnées mises à jour
        let updated_metadata_json = serde_json::to_string(&updated_metadata).unwrap();
        fs::write(metadata_path, updated_metadata_json).ok();

        return Err(SmartLockerError::DecryptionError(format!(
            "The secret '{}' has expired. Please renew it to use it again.",
            name
        )));
    }

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
