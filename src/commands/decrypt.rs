use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM
use aes_gcm::aead::Aead; // Import du trait pour le chiffrement
use aes_gcm::KeyInit; // Import du trait nécessaire pour initialiser le chiffreur
use std::fs;
use directories::UserDirs;
use flate2::read::GzDecoder;
use std::io::Read;

pub fn decrypt(name: &str) -> String {
    let user_dirs = UserDirs::new().expect("Unable to access user directory");
    let locker_dir = user_dirs.home_dir().join(".locker");
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).expect("Unable to read symmetric key");
    let key = Key::<Aes256Gcm>::from_slice(&key_data);

    // Initialiser AES-GCM avec la clé
    let cipher = Aes256Gcm::new(key);

    // Lire le fichier chiffré
    let input_path = locker_dir.join(format!("{}.slock", name));
    let encrypted_data = fs::read(&input_path).expect("Unable to read encrypted file");

    // Extraire le nonce et les données chiffrées
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    // Déchiffrer les données
    let decrypted_data = cipher
        .decrypt(nonce, ciphertext)
        .expect("Error during decryption");

    // Décompresser les données
    let mut decoder = GzDecoder::new(&decrypted_data[..]);
    let mut decompressed_data = String::new();
    decoder
        .read_to_string(&mut decompressed_data)
        .expect("Error during data decompression");

    decompressed_data
}