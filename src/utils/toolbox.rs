use colored::Colorize;
use directories::UserDirs;
use ring::pbkdf2;
use std::env;
use std::fs;
use std::num::NonZeroU32;
use std::path::PathBuf;

/// Initialise le répertoire `.locker` et génère une clé symétrique si nécessaire.
pub fn init_locker() -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    // Check if the locker directory exists
    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error creating folder ~/.locker: {}", e))
        })?;
        println!("✅ Secure folder created: {:?}", locker_dir);
    }

    // Check if the key file exists
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
}

pub fn init_locker_with_passphrase(passphrase: Option<&str>) {
    let locker_dir = get_locker_dir();
    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).expect("Error creating folder ~/.locker");
        println!("✅ Secure folder created: {:?}", locker_dir);
    }

    let key_path = locker_dir.join("locker.key");

    if let Some(passphrase) = passphrase {
        let salt = b"smartlocker_salt"; // Vous pouvez personnaliser le sel
        let new_key = derive_key_from_passphrase(passphrase, salt);

        if key_path.exists() {
            println!("🔑 Une clé existe déjà : {:?}", key_path);
            println!("⚠️ Attention : Générer une nouvelle clé remplacera l'ancienne et rendra les anciens secrets inaccessibles.");
            println!("Voulez-vous continuer ? (yes/no)");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Erreur lors de la lecture de l'entrée utilisateur");
            if input.trim().to_lowercase() != "yes" {
                println!("❌ Opération annulée.");
                return;
            }
        }

        fs::write(&key_path, new_key).expect("Erreur lors de l'écriture de la clé");
        println!(
            "{}",
            format!(
                "✅ Nouvelle clé générée à partir de la passphrase et sauvegardée : {:?}",
                key_path
            )
            .green()
        );
    } else {
        init_locker(); // Appelle la fonction existante pour générer une clé aléatoire
    }
}

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

/// Retourne le chemin du répertoire `.locker`.
pub fn get_locker_dir() -> Result<PathBuf, SmartLockerError> {
    if let Ok(custom_home) = env::var("SMART_LOCKER_HOME") {
        PathBuf::from(custom_home)
    } else {
        let user_dirs = UserDirs::new().expect("Unable to access user directory");
        user_dirs.home_dir().join(".locker")
    }
}

pub fn backup_key() {
    let locker_dir = get_locker_dir();
    let key_path = locker_dir.join("locker.key");
    let backup_path = locker_dir.join("locker.key.backup");

    if key_path.exists() {
        fs::copy(&key_path, &backup_path).expect("Erreur lors de la sauvegarde de la clé");
        println!("✅ Clé sauvegardée avec succès : {:?}", backup_path);
    } else {
        println!("❌ Aucune clé à sauvegarder.");
    }
}

pub fn restore_key() {
    let locker_dir = get_locker_dir();
    let key_path = locker_dir.join("locker.key");
    let backup_path = locker_dir.join("locker.key.backup");

    if backup_path.exists() {
        fs::copy(&backup_path, &key_path).expect("Erreur lors de la restauration de la clé");
        println!("✅ Clé restaurée avec succès : {:?}", key_path);
    } else {
        println!("❌ Aucune sauvegarde de clé trouvée.");
    }
}
