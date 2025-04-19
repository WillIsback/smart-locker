use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;

pub fn decrypt(name: &str) -> Result<String, SmartLockerError> {
    let locker_dir = get_locker_dir()?;
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).expect("Impossible de lire la clé symétrique");
    let key = Key::<Aes256Gcm>::from_slice(&key_data);

    // Initialiser AES-GCM avec la clé
    let cipher = Aes256Gcm::new(key);

    // Lire le fichier chiffré
    let input_path = locker_dir.join(format!("{}.slock", name));
    let encrypted_data = fs::read(&input_path).expect("Impossible de lire le fichier chiffré");

    // Extraire le nonce et les données chiffrées
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce);

    // Déchiffrer les données
    let decrypted_data = match cipher.decrypt(nonce, ciphertext) {
        Ok(data) => data,
        Err(_) => {
            eprintln!(
                "❌ Erreur : La clé de chiffrement est incorrecte ou le fichier est corrompu."
            );
            std::process::exit(1); // Quitte le programme avec un code d'erreur
        }
    };

    // Décompresser les données
    let mut decoder = GzDecoder::new(&decrypted_data[..]);
    let mut decompressed_data = String::new();
    decoder
        .read_to_string(&mut decompressed_data)
        .expect("Erreur lors de la décompression des données");
    decompressed_data
}
