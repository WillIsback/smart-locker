use directories::UserDirs;
use serial_test::serial;
use smart_locker::commands::{decrypt, encrypt, export, init, list, remove, renew};
use std::env;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

// Helper function to setup and initialize the test environment
fn setup_and_initialize() -> PathBuf {
    let unique_id = Uuid::new_v4().to_string();
    let test_dir = UserDirs::new()
        .map(|dirs| dirs.home_dir().join(".locker/test/").join(&unique_id))
        .expect("Failed to get user directories");

    let env_var_name = format!("SMART_LOCKER_TEST_DIR_{}", unique_id);
    env::set_var(&env_var_name, &test_dir);

    if test_dir.exists() {
        // Nettoyer le dossier de test
        fs::remove_dir_all(test_dir.clone()).unwrap_or_else(|e| {
            eprintln!("Failed to clean up test directory: {}", e);
        });
    } else {
        init::init_locker_with_passphrase(None)
            .expect("Failed to initialize locker with passphrase");
    }
    test_dir
}

// Helper function to clean up environment variables
fn cleanup_environment_variables() {
    for (key, _) in env::vars() {
        if key.starts_with("SMART_LOCKER_TEST_DIR_") {
            env::remove_var(key);
        }
    }
}

#[cfg(test)]
mod cleanup {
    use std::fs;

    /// Nettoie tout le dossier `test` après l'exécution de tous les tests.
    #[ctor::dtor]
    fn cleanup_test_directory() {
        let test_dir = directories::UserDirs::new()
            .expect("Failed to get user directories")
            .home_dir()
            .join(".locker/test");

        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).unwrap_or_else(|e| {
                eprintln!("Failed to clean up test directory: {}", e);
            });
            println!("✅ Cleaned up test directory: {:?}", test_dir);
        }
    }
}

#[test]
#[serial]
fn test_encrypt_and_decrypt() {
    let locker_dir = setup_and_initialize();
    let secret_name = "test_encrypt_and_decrypt_secret";
    let secret_value = "Ceci est un test";
    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));

    // Encrypt the secret
    let tags: Vec<String> = ["test1", "test2"].iter().map(|&s| s.to_string()).collect();
    let expiration_days = Some(30);
    encrypt::encrypt(secret_value, secret_name, tags, expiration_days)
        .expect("Failed to encrypt secret");

    // Verify encrypted file exists
    assert!(
        encrypted_file.exists(),
        "Encrypted file was not created: {:?}",
        encrypted_file
    );

    // Decrypt and verify content
    let decrypted_value = decrypt::decrypt(secret_name).expect("Failed to decrypt secret");
    println!("Decrypted value: {}", decrypted_value);
    assert_eq!(
        decrypted_value, secret_value,
        "Decrypted secret doesn't match original"
    );
    // Nettoyage des variables d'environnement
    cleanup_environment_variables();
}

#[test]
#[serial]
fn test_list_secrets() {
    let locker_dir = setup_and_initialize();
    let test_secret_name = "test_list_secrets_secret";
    let test_secret_value = "This is a test secret";

    // Encrypt the secret (this ensures metadata is created)
    encrypt::encrypt(
        test_secret_value,
        test_secret_name,
        vec!["test".to_string()],
        Some(30),
    )
    .expect("Failed to encrypt secret");

    // Verify the encrypted file exists
    let test_file = locker_dir.join(format!("{}.slock", test_secret_name));
    assert!(test_file.exists(), "Test file was not created properly");

    // Get the list of secret names
    let secret_names = list::list_secrets_names().expect("Failed to list secret names");

    // Print the list of secrets
    println!("List of secrets: {:?}", secret_names);

    // Verify our test secret is in the list
    assert!(
        secret_names.contains(&test_secret_name.to_string()),
        "The secret '{}' is not listed. Secrets: {:?}",
        test_secret_name,
        secret_names
    );
    // Nettoyage des variables d'environnement
    cleanup_environment_variables();
}

#[test]
#[serial]
fn test_remove_secret() {
    let locker_dir = setup_and_initialize();
    let test_secret_name = "test_remove_secret_secret";
    let test_secret_value = "This is a test secret";

    // Encrypt the secret (this ensures metadata is created)
    encrypt::encrypt(
        test_secret_value,
        test_secret_name,
        vec!["test".to_string()],
        Some(30),
    )
    .expect("Failed to encrypt secret");

    // Verify the encrypted file exists
    let test_file = locker_dir.join(format!("{}.slock", test_secret_name));
    assert!(test_file.exists(), "Test file was not created properly");

    // List files in the directory for debugging
    for entry in fs::read_dir(&locker_dir).expect("Failed to read locker directory") {
        let entry = entry.expect("Failed to read entry");
        println!("Found file: {:?}", entry.path());
    }

    // Remove the secret
    remove::remove_secret(Some(test_secret_name), false).expect("Failed to remove secret");

    // Verify the file is gone
    assert!(!test_file.exists(), "Secret file wasn't removed");

    // Verify the metadata is gone
    let metadata =
        fs::read_to_string(locker_dir.join("metadata.json")).expect("Failed to read metadata file");
    println!("Metadata after removal: {}", metadata);
    assert!(
        !metadata.contains(test_secret_name),
        "Metadata for the secret was not removed"
    );
    // Nettoyage des variables d'environnement
    cleanup_environment_variables();
}

#[test]
#[serial]
fn test_export_secrets() {
    let locker_dir = setup_and_initialize();
    let secret_name = "test_export_secret";
    let secret_value = "export_test_value";
    let export_file = locker_dir.join("exported_secrets.env");

    // Encrypt a secret
    let tags: Vec<String> = ["export", "test"].iter().map(|&s| s.to_string()).collect();
    encrypt::encrypt(secret_value, secret_name, tags, None).expect("Failed to encrypt secret");

    // Export secrets
    export::export("env", export_file.to_str()).expect("Failed to export secrets");

    // Verify export file exists
    assert!(
        export_file.exists(),
        "Export file was not created: {:?}",
        export_file
    );

    // Verify content of the export file
    let exported_content = fs::read_to_string(&export_file).expect("Failed to read export file");
    println!("Exported file content: \n{}", exported_content);
    assert!(
        exported_content.contains(&format!(
            "{}=$(smart-locker decrypt -n {})",
            secret_name, secret_name
        )),
        "Exported content is incorrect: {}",
        exported_content
    );
    // Nettoyage des variables d'environnement
    cleanup_environment_variables();
}

#[test]
#[serial]
fn test_renew_secret() {
    let locker_dir = setup_and_initialize();
    let secret_name = "test_renew_secret";
    let secret_value = "renew_test_value";

    // Encrypt a secret
    let tags: Vec<String> = ["renew", "test"].iter().map(|&s| s.to_string()).collect();
    encrypt::encrypt(secret_value, secret_name, tags, Some(1)).expect("Failed to encrypt secret");

    // Print expiration before renewal
    let metadata =
        fs::read_to_string(locker_dir.join("metadata.json")).expect("Failed to read metadata file");
    println!("Metadata before renewal: {}", metadata);

    // Renew the secret
    renew::renew_secret(secret_name, 30).expect("Failed to renew secret");

    // Print expiration after renewal
    let updated_metadata = fs::read_to_string(locker_dir.join("metadata.json"))
        .expect("Failed to read updated metadata file");
    println!("Metadata after renewal: {}", updated_metadata);

    // Verify the secret is still decryptable
    let decrypted_value = decrypt::decrypt(secret_name).expect("Failed to decrypt renewed secret");
    assert_eq!(
        decrypted_value, secret_value,
        "Decrypted value doesn't match original after renewal"
    );
    // Nettoyage des variables d'environnement
    cleanup_environment_variables();
}
