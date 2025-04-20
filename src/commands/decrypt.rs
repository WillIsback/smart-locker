use crate::utils::metadata::{read_metadata, has_this_secret_metadata, is_secret_expired, mark_secret_as_expired};
use crate::utils::toolbox::{get_locker_dir, is_this_secret};
use crate::commands::migrate::migrate_metadata;
use crate::MetadataFile;
use crate::LockerResult;
use colored::Colorize;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::read::GzDecoder;
use std::fs;
use std::io::{self, Read, Write};


pub fn decrypt(name: &str) -> LockerResult<String> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");
    let secret_path = locker_dir.join(format!("{}.slock", name));

    // Vérifier si le fichier est un secret valide
    let (is_valid, _secret_name) = is_this_secret(&secret_path, false);
    if !is_valid {
        return Err(SmartLockerError::FileSystemError(format!(
            "The file '{}' is not a valid secret file.",
            secret_path.display()
        )));
    }

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

        // Lire la saisie utilisateur
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "yes" {
            println!(
                "{}",
                format!("Migrating metadata for secret '{}'...", name).blue()
            );
            migrate_metadata(Some(name))?;
            println!("{}", "✅ Metadata migration completed successfully.".green());
            metadata = read_metadata()?; // Relire les métadonnées après migration
        } else {
            return Err(SmartLockerError::DecryptionError(format!(
                "Metadata for secret '{}' is invalid. Migration was skipped.",
                name
            )));
        }
    }

    // Vérifier si le secret est expiré
    let secret_metadata = metadata.secrets.get(name).ok_or_else(|| {
        SmartLockerError::DecryptionError(format!(
            "Metadata for secret '{}' is missing after migration.",
            name
        ))
    })?;

    if is_secret_expired(secret_metadata) {
        mark_secret_as_expired(name, &mut metadata)?;
        return Err(SmartLockerError::DecryptionError(format!(
            "The secret '{}' has expired. Please renew it to use it again.",
            name
        )));
    }

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to read the symmetric key".to_string())
    })?;
    println!("DEBUG: Key length: {}", key_data.len());
    let key = Key::<Aes256Gcm>::from_slice(&key_data);

    // Initialiser AES-GCM avec la clé
    let cipher = Aes256Gcm::new(key);

    // Lire le fichier chiffré
    let encrypted_data = fs::read(&secret_path).map_err(|_| {
        SmartLockerError::FileSystemError("Unable to read the encrypted file".to_string())
    })?;
    println!("DEBUG: Encrypted data length: {}", encrypted_data.len());

    // Extraire le nonce et les données chiffrées
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce);


    // Déchiffrer les données
    let decrypted_data = cipher
    .decrypt(nonce, ciphertext)
    .map_err(|_| {
        println!("DEBUG: Decryption failed.");
        SmartLockerError::DecryptionError("Decryption failed".to_string())
    })?;
    println!("DEBUG: Decrypted data (raw): {:?}", decrypted_data);
    
    // Décompresser les données
    let mut decoder = GzDecoder::new(&decrypted_data[..]);
    let mut decompressed_data = String::new();
    decoder
        .read_to_string(&mut decompressed_data)
        .map_err(|_| {
            println!("DEBUG: Decompression failed.");
            SmartLockerError::FileSystemError("Failed to decompress the data".to_string())
        })?;
    println!("DEBUG: Decompressed data length: {}", decompressed_data.len());
    println!("DEBUG: Decompressed data: {}", decompressed_data);
    Ok(decompressed_data)
}