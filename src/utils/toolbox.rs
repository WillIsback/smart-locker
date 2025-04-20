use crate::LockerResult;
use crate::SmartLockerError;
use colored::Colorize;
use directories::UserDirs;
use ring::pbkdf2;
use std::env;
use std::fs;
use std::num::NonZeroU32;
use std::path::PathBuf;
use copypasta::{ClipboardContext, ClipboardProvider};

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

/// Vérifie et crée un répertoire s'il n'existe pas.
pub fn ensure_dir_exists(path: &PathBuf) -> LockerResult<()> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error creating folder {:?}: {}", path, e))
        })?;
        println!("✅ Directory created: {:?}", path);
    }
    Ok(())
}

/// Retourne le chemin du répertoire `.locker`.
pub fn get_locker_dir() -> LockerResult<PathBuf> {
    if let Ok(custom_home) = env::var("SMART_LOCKER_HOME") {
        Ok(PathBuf::from(custom_home))
    } else {
        let user_dirs = UserDirs::new().ok_or_else(|| {
            SmartLockerError::FileSystemError("Unable to access user directory".to_string())
        })?;
        Ok(user_dirs.home_dir().join(".locker"))
    }
}

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

    Ok(()) // Return success
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

/// Vérifie si le fichier donné est un secret valide avec l'extension `.slock`.
///
/// # Arguments
///
/// * `file_path` - Le chemin du fichier à vérifier.
/// * `silent` - Si `true`, la fonction n'affiche aucun message.
///
/// # Retourne
///
/// * `(bool, Option<String>)` - Un tuple où :
///   - `bool` indique si le fichier est un secret valide.
///   - `Option<String>` contient le nom du secret (`secret_name`) si le fichier est valide.
///
/// # Exemple
///
/// ```rust
/// use std::path::PathBuf;
/// let file_path = PathBuf::from("example.slock");
/// let (is_valid, secret_name) = is_this_secret(&file_path, false);
/// if is_valid {
///     println!("Secret valid: {}", secret_name.unwrap());
/// }
/// ```
pub fn is_this_secret(file_path: &PathBuf, silent: bool) -> (bool, Option<String>) {
    if file_path.extension().and_then(|ext| ext.to_str()) == Some("slock") {
        if let Some(secret_name) = file_path.file_stem().and_then(|stem| stem.to_str()) {
            return (true, Some(secret_name.to_string()));
        } else if !silent {
            println!(
                "{}",
                format!("⚠️ Invalid secret file name '{}'.", file_path.display()).yellow()
            );
        }
        (false, None)
    } else {
        if !silent {
            println!(
                "{}",
                format!(
                    "⚠️ The file '{}' is not a valid secret file.",
                    file_path.display()
                )
                .yellow()
            );
        }
        (false, None)
    }
}


/// Copie une chaîne de caractères dans le presse-papiers.
/// Retourne une erreur si l'opération échoue.
pub fn copy_to_clipboard(content: &str) -> Result<(), String> {
    let mut ctx = ClipboardContext::new().map_err(|_| "Unable to access the clipboard".to_string())?;
    ctx.set_contents(content.to_string())
        .map_err(|_| "Failed to copy content to the clipboard".to_string())
}