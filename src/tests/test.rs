use std::fs;
use std::io::Read;
use directories::UserDirs;
use smart_locker::commands::{encrypt, decrypt, list, remove};
use smart_locker::utils::derive_key_from_passphrase;

#[test]
fn test_derive_key_from_passphrase() {
    let passphrase = "ma_passphrase";
    let salt = b"mon_salt";
    let key = derive_key_from_passphrase(passphrase, salt);

    assert_eq!(key.len(), 32, "La clé dérivée doit avoir une longueur de 32 octets");
}

#[test]
fn test_encrypt_and_decrypt() {
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");
    let key_path = locker_dir.join("locker.key");

    // Vérifier et créer le dossier sécurisé
    if !locker_dir.exists() {
        println!("Création du dossier sécurisé : {:?}", locker_dir);
        fs::create_dir_all(&locker_dir).expect("Erreur lors de la création du dossier ~/.locker");
    }

    // Vérifier et créer la clé
    if !key_path.exists() {
        println!("Création de la clé : {:?}", key_path);
        let key = vec![0u8; 32];
        fs::write(&key_path, key).expect("Erreur lors de l'écriture de la clé");
    }

    let secret_name = "test_encrypt_and_decrypt_secret";
    let secret_value = "Ceci est un test";

    // Appeler la fonction encrypt
    println!("Chiffrement du secret : {}", secret_name);
    encrypt::encrypt(secret_value, secret_name);

    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));
    println!("Vérification de l'existence du fichier chiffré : {:?}", encrypted_file);

    // Vérifier que le fichier chiffré a été créé
    assert!(
        encrypted_file.exists(),
        "Le fichier chiffré n'a pas été créé : {:?}",
        encrypted_file
    );

    // Déchiffrer le secret
    let decrypted_value = decrypt::decrypt(secret_name);
    println!("Valeur déchiffrée : {}", decrypted_value);

    // Vérifier que la valeur déchiffrée correspond à la valeur initiale
    assert_eq!(
        decrypted_value, secret_value,
        "Le secret déchiffré ne correspond pas"
    );

    // Nettoyer les fichiers de test
    fs::remove_file(&encrypted_file).expect("Erreur lors de la suppression du fichier chiffré");
}

#[test]
fn test_list_secrets() {
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).expect("Erreur lors de la création du dossier ~/.locker");
    }

    let test_file = locker_dir.join("test_list_secrets_secret.slock");
    fs::write(&test_file, b"test").expect("Erreur lors de la création du fichier de test");

    let secrets = list::list_secrets(&locker_dir);

    assert!(
        secrets.contains(&"test_list_secrets_secret.slock".to_string()),
        "Le fichier 'test_list_secrets_secret.slock' n'apparaît pas dans la liste. Secrets : {:?}",
        secrets
    );

    fs::remove_file(&test_file).expect("Erreur lors de la suppression du fichier de test");
}

#[test]
fn test_remove_secret() {
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    let test_file = locker_dir.join("test_remove_secret_secret.slock");
    fs::write(&test_file, b"test").expect("Erreur lors de la création du fichier de test");

    remove::remove_secret("test_remove_secret_secret");

    assert!(!test_file.exists(), "Le fichier n'a pas été supprimé");
}

#[test]
fn test_encrypt_with_stdin() {
    use std::io::Cursor;

    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    // Créer un fichier temporaire pour simuler stdin
    let secret_name = "test_encrypt_with_stdin_secret";
    let secret_value = "Ceci est un test";

    // Simuler l'entrée stdin avec un Cursor
    let mut stdin_mock = Cursor::new(secret_value);

    // Lire depuis le buffer simulé (au lieu de std::io::stdin())
    let mut input = String::new();
    stdin_mock
        .read_to_string(&mut input)
        .expect("Erreur lors de la lecture de stdin simulé");

    // Appeler la fonction encrypt avec la valeur lue
    encrypt::encrypt(&input, secret_name);

    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));
    assert!(
        encrypted_file.exists(),
        "Le fichier chiffré n'a pas été créé : {:?}",
        encrypted_file
    );

    // Nettoyer les fichiers de test
    fs::remove_file(&encrypted_file).expect("Erreur lors de la suppression du fichier de test");
}


#[test]
fn test_decrypt_with_stdout() {
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    let secret_name = "test_decrypt_with_stdout_secret";
    let secret_value = "Ceci est un test";

    // Chiffrer le secret
    encrypt::encrypt(secret_value, secret_name);

    // Déchiffrer le secret
    let decrypted_value = decrypt::decrypt(secret_name);

    assert_eq!(
        decrypted_value, secret_value,
        "Le secret déchiffré ne correspond pas à la valeur initiale"
    );

    // Nettoyer les fichiers de test
    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));
    fs::remove_file(&encrypted_file).expect("Erreur lors de la suppression du fichier de test");
}

#[test]
fn test_decrypt_with_clipboard() {
    use copypasta::{ClipboardContext, ClipboardProvider};

    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    let secret_name = "test_decrypt_with_clipboard_secret";
    let secret_value = "Ceci est un test";

    // Chiffrer le secret
    encrypt::encrypt(secret_value, secret_name);

    // Déchiffrer le secret et copier dans le presse-papier
    let decrypted_value = decrypt::decrypt(secret_name);
    let mut ctx = ClipboardContext::new().expect("Impossible d'accéder au presse-papier");
    ctx.set_contents(decrypted_value.clone())
        .expect("Erreur lors de la copie dans le presse-papier");

    // Vérifier que le contenu du presse-papier est correct
    let clipboard_content = ctx.get_contents().expect("Erreur lors de la lecture du presse-papier");
    assert_eq!(
        clipboard_content, secret_value,
        "Le contenu du presse-papier ne correspond pas au secret déchiffré"
    );

    // Nettoyer les fichiers de test
    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));
    fs::remove_file(&encrypted_file).expect("Erreur lors de la suppression du fichier de test");
}