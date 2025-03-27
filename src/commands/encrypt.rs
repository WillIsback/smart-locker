use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM
use aes_gcm::KeyInit; // Import du trait nécessaire pour initialiser le chiffreur
use aes_gcm::aead::Aead; // Import du trait pour le chiffrement
use std::fs;
use directories::UserDirs;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

pub fn encrypt(secret: &str, name: &str) {
    // Obtenir le chemin du dossier sécurisé
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");
    let key_path = locker_dir.join("locker.key");

    // Lire la clé symétrique
    let key_data = fs::read(&key_path).expect("Impossible de lire la clé symétrique");
    let key = Key::<Aes256Gcm>::from_slice(&key_data); // Spécifiez explicitement le type de clé

    // Initialiser AES-GCM avec la clé
    let cipher = Aes256Gcm::new(key);

    // Générer un nonce aléatoire (12 octets)
    let random_bytes = rand::random::<[u8; 12]>(); // Stocker les octets générés dans une variable
    let nonce = Nonce::from_slice(&random_bytes);  // Passer une référence à la variable

    // Compresser le secret
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(secret.as_bytes())
        .expect("Erreur lors de la compression des données");
    let compressed_data = encoder
        .finish()
        .expect("Erreur lors de la finalisation de la compression");

    // Chiffrer les données compressées
    let ciphertext = cipher
        .encrypt(&nonce, compressed_data.as_ref())
        .expect("Erreur lors du chiffrement");

    // Sauvegarder le secret chiffré dans un fichier `.slock`
    let output_path = locker_dir.join(format!("{}.slock", name));
    let mut output_data = Vec::new();
    output_data.extend_from_slice(nonce);
    output_data.extend_from_slice(&ciphertext);

    fs::write(&output_path, output_data).expect("Erreur lors de l'écriture du fichier chiffré");

    println!("✅ Secret chiffré et sauvegardé dans : {:?}", output_path);
}