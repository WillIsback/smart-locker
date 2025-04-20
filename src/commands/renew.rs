use crate::utils::metadata::{has_this_secret_metadata,is_secret_expired, read_metadata, mark_secret_as_expired, update_secret_expiration};
use crate::MetadataFile;
use crate::SmartLockerError;
use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn renew_secret(name: &str, additional_days: u64) -> Result<(), SmartLockerError> {
    // Charger les métadonnées
    let metadata_result = read_metadata();
    let mut metadata = metadata_result.unwrap_or_else(|_| MetadataFile {
        secrets: Default::default(),
    });
    // Vérifier les métadonnées
    if !has_this_secret_metadata(name, &metadata) {
        println!(
            "{}",
            format!(
                "⚠️ Metadata for secret '{}' is missing or outdated. Do you want to migrate it? (yes/no): ",
                name
            )
            .yellow()
        );
    }
    // Vérifier si le secret est présent dans les métadonnées
    let secret_metadata = metadata.secrets.get(name).ok_or_else(|| {
        SmartLockerError::DecryptionError(format!(
            "Metadata for secret '{}' is missing after migration.",
            name
        ))
    })?;

    // Vérifier si le secret est expiré
    if is_secret_expired(secret_metadata) {
        mark_secret_as_expired(name, &mut metadata)?;
        return Err(SmartLockerError::DecryptionError(format!(
            "The secret '{}' has expired. Please renew it to use it again.",
            name
        )));
    }

    // Renouveler la clé
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let new_expiration = now + (additional_days * 24 * 60 * 60);

    update_secret_expiration(name, &mut metadata, new_expiration)?;

    println!("✅ The secret '{}' has been successfully renewed.", name);
    Ok(())
}
