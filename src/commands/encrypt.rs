// src/commands/encrypt.rs

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Assuming aes-gcm is used for encryption
use rand::Rng;
use serde::{Serialize, Deserialize};

const LOCKER_DIR: &str = "~/.locker/";

#[derive(Serialize, Deserialize)]
struct Secret {
    name: String,
    value: String,
}

pub fn encrypt_secret(name: &str, value: &str) -> io::Result<()> {
    let locker_path = shellexpand::tilde(LOCKER_DIR).to_string();
    let file_path = format!("{}.slock", locker_path);
    
    // Generate a random nonce
    let nonce: [u8; 12] = rand::thread_rng().gen();
    
    // Load the encryption key (this should be securely managed)
    let key = Key::from_slice(&load_key()?); // Assuming load_key() retrieves the key

    // Create the cipher
    let cipher = Aes256Gcm::new(&key);
    
    // Encrypt the secret
    let encrypted_data = cipher.encrypt(Nonce::from_slice(&nonce), value.as_bytes())
        .expect("Encryption failed");

    // Write the encrypted data to the file
    let mut file = File::create(file_path)?;
    file.write_all(&nonce)?;
    file.write_all(&encrypted_data)?;

    Ok(())
}

// Placeholder for the load_key function
fn load_key() -> io::Result<Vec<u8>> {
    // Implement key loading logic here
    Ok(vec![0; 32]) // Example: return a dummy key
}