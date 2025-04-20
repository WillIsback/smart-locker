use crate::utils::metadata::init_metadata_file;
use crate::utils::toolbox::{ensure_dir_exists, get_locker_dir};
use crate::LockerResult;
use crate::SmartLockerError;
use colored::Colorize;
use ring::pbkdf2;
use std::fs;
use std::num::NonZeroU32;

pub fn init_locker_with_passphrase(passphrase: Option<&str>) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?; // `?` propagates the error as a `Result`

    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).expect("Error creating folder ~/.locker");
        println!("✅ Secure folder created: {:?}", locker_dir);
    }

    let key_path = locker_dir.join("locker.key");

    if let Some(passphrase) = passphrase {
        let salt = b"smartlocker_salt"; // Customize the salt
        let new_key = derive_key_from_passphrase(passphrase, salt)?; // `?` propagates errors

        if key_path.exists() {
            println!("🔑 A key already exists: {:?}", key_path);
            println!("⚠️ Warning: Generating a new key will replace the old one and make old secrets inaccessible.");
            println!("Do you want to continue? (yes/no)");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error reading user input");
            if input.trim().to_lowercase() != "yes" {
                println!("❌ Operation canceled.");
                return Ok(()); // Return early with `Ok(())`
            }
        }

        fs::write(&key_path, new_key).expect("Error writing the key");
        println!(
            "{}",
            format!(
                "✅ New key generated from the passphrase and saved: {:?}",
                key_path
            )
            .green()
        );
    } else {
        init_locker()?; // Call another function that returns `Result`
    }
    // Initialiser le fichier metadata.json
    init_metadata_file().expect("Error initializing metadata file");

    Ok(()) // Return success
}

/// Initialise le répertoire `.locker` et génère une clé symétrique si nécessaire.
pub fn init_locker() -> LockerResult<()> {
    let locker_dir = get_locker_dir()?;
    ensure_dir_exists(&locker_dir)?;

    let key_path = locker_dir.join("locker.key");
    if !key_path.exists() {
        let key = generate_key();
        fs::write(&key_path, key).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error writing the key: {}", e))
        })?;
        println!("✅ Key generated and saved: {:?}", key_path);
    } else {
        println!("🔑 A key already exists: {:?}", key_path);
    }

    Ok(())
}

/// Génère une clé symétrique aléatoire.
pub fn generate_key() -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    key.to_vec()
}

/// Generates a symmetric key from a passphrase and salt.
pub fn derive_key_from_passphrase(
    passphrase: &str,
    salt: &[u8],
) -> Result<Vec<u8>, SmartLockerError> {
    let locker_dir = get_locker_dir()?;

    // Check if the locker directory exists
    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error creating folder ~/.locker: {}", e))
        })?;
        println!("✅ Secure folder created: {:?}", locker_dir);
    }

    let mut key = [0u8; 32]; // 32-byte key
    let iterations = NonZeroU32::new(100_000).unwrap(); // Number of PBKDF2 iterations
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        passphrase.as_bytes(),
        &mut key,
    );

    Ok(key.to_vec())
}

/// Sauvegarde la clé de chiffrement.
pub fn backup_key() -> LockerResult<()> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");
    let backup_path = locker_dir.join("locker.key.backup");

    if key_path.exists() {
        fs::copy(&key_path, &backup_path).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error backing up key: {}", e))
        })?;
        println!("✅ Key backed up successfully: {:?}", backup_path);
    } else {
        println!("❌ No key to back up.");
    }
    Ok(())
}

/// Restaure la clé de chiffrement à partir d'une sauvegarde.
pub fn restore_key() -> LockerResult<()> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");
    let backup_path = locker_dir.join("locker.key.backup");

    if backup_path.exists() {
        fs::copy(&backup_path, &key_path).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error restoring key: {}", e))
        })?;
        println!("✅ Key restored successfully: {:?}", key_path);
    } else {
        println!("❌ No backup key found.");
    }
    Ok(())
}
