use std::fs;
use ring::pbkdf2;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::env;

// Cette fonction initialise le coffre-fort en créant un dossier sécurisé et en générant une clé symétrique.
pub fn init_locker() {
    // Vérifier si une variable d'environnement définit le chemin du répertoire
    let locker_dir: PathBuf = if let Ok(custom_home) = env::var("SMART_LOCKER_HOME") {
        PathBuf::from(custom_home)
    } else {
        let user_dirs = directories::UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
        user_dirs.home_dir().join(".locker")
    };
    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).expect("Erreur lors de la création du dossier ~/.locker");
        println!("✅ Dossier sécurisé créé : {:?}", locker_dir);
    }

    let key_path = locker_dir.join("locker.key");
    if !key_path.exists() {
        let key = generate_key();
        fs::write(&key_path, key).expect("Erreur lors de l'écriture de la clé");
        println!("✅ Clé générée et sauvegardée : {:?}", key_path);
    } else {
        println!("🔑 Une clé existe déjà : {:?}", key_path);
    }

}

pub fn generate_key() -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    key.to_vec()
}

/// Génère une clé symétrique à partir d'une passphrase et d'un sel.
pub fn derive_key_from_passphrase(passphrase: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = [0u8; 32]; // Clé de 32 octets
    let iterations = NonZeroU32::new(100_000).unwrap(); // Nombre d'itérations PBKDF2
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        passphrase.as_bytes(),
        &mut key,
    );
    key.to_vec()
}