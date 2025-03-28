use std::fs;
use ring::pbkdf2;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::env;

// This function initializes the vault by creating a secure folder and generating a symmetric key.
pub fn init_locker() {
    // Check if an environment variable defines the directory path
    let locker_dir: PathBuf = if let Ok(custom_home) = env::var("SMART_LOCKER_HOME") {
        PathBuf::from(custom_home)
    } else {
        let user_dirs = directories::UserDirs::new().expect("Unable to access user folder");
        user_dirs.home_dir().join(".locker")
    };
    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).expect("Error creating folder ~/.locker");
        println!("✅ Secure folder created: {:?}", locker_dir);
    }

    let key_path = locker_dir.join("locker.key");
    if !key_path.exists() {
        let key = generate_key();
        fs::write(&key_path, key).expect("Error writing the key");
        println!("✅ Key generated and saved: {:?}", key_path);
    } else {
        println!("🔑 A key already exists: {:?}", key_path);
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
pub fn derive_key_from_passphrase(passphrase: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = [0u8; 32]; // 32-byte key
    let iterations = NonZeroU32::new(100_000).unwrap(); // Number of PBKDF2 iterations
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        passphrase.as_bytes(),
        &mut key,
    );
    key.to_vec()
}