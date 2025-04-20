use crate::utils::toolbox::get_locker_dir;
use crate::SecretMetadata;
use crate::SmartLockerError;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn renew_secret(name: &str, additional_days: u64) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let metadata_path = locker_dir.join(format!("{}.meta.json", name));

    // Lire les métadonnées
    let metadata_content = fs::read_to_string(&metadata_path).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to read metadata file".to_string())
    })?;
    let mut metadata: SecretMetadata = serde_json::from_str(&metadata_content).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to parse metadata file".to_string())
    })?;

    // Vérifier si la clé est expirée
    if !metadata.expired {
        return Err(SmartLockerError::DecryptionError(format!(
            "The secret '{}' is not expired and does not need renewal.",
            name
        )));
    }

    // Renouveler la clé
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    metadata.expire_at = now + (additional_days * 24 * 60 * 60);
    metadata.expired = false;

    // Sauvegarder les métadonnées mises à jour
    let updated_metadata_json = serde_json::to_string(&metadata).unwrap();
    fs::write(metadata_path, updated_metadata_json).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to write updated metadata file".to_string())
    })?;

    println!("✅ The secret '{}' has been successfully renewed.", name);
    Ok(())
}
