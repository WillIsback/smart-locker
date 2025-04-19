use smart_locker::commands::{decrypt, encrypt, list, remove};
use smart_locker::utils::toolbox::get_locker_dir;
use std::fs;
use std::io::Read;

// Helper function to setup test environment
fn setup_test_environment() -> std::path::PathBuf {
    let locker_dir = get_locker_dir().expect("Failed to get locker directory");
    let key_path = locker_dir.join("locker.key");

    // Create locker directory if it doesn't exist
    if !locker_dir.exists() {
        fs::create_dir_all(&locker_dir).expect("Failed to create locker directory");
    }

    // Create key file if it doesn't exist
    if !key_path.exists() {
        let key = vec![0u8; 32];
        fs::write(&key_path, key).expect("Failed to write key file");
    }

    locker_dir
}

// Helper function to clean up test files
fn cleanup_test_file(filename: &str) {
    let path = get_locker_dir()
        .expect("Failed to get locker directory")
        .join(filename);
    if path.exists() {
        fs::remove_file(&path).unwrap_or_else(|e| eprintln!("Cleanup error: {}", e));
    }
}

#[test]
fn test_encrypt_and_decrypt() {
    let locker_dir = setup_test_environment();

    let secret_name = "test_encrypt_and_decrypt_secret";
    let secret_value = "Ceci est un test";
    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));

    // Ensure the test file doesn't exist before starting
    cleanup_test_file(&format!("{}.slock", secret_name));

    // Encrypt the secret
    encrypt::encrypt(secret_value, secret_name).expect("Failed to encrypt secret");

    // Verify encrypted file exists
    assert!(
        encrypted_file.exists(),
        "Encrypted file was not created: {:?}",
        encrypted_file
    );

    // Decrypt and verify content
    let decrypted_value = decrypt::decrypt(secret_name).expect("Failed to decrypt secret");
    assert_eq!(
        decrypted_value, secret_value,
        "Decrypted secret doesn't match original"
    );

    // Clean up test file
    cleanup_test_file(&format!("{}.slock", secret_name));
}

#[test]
fn test_list_secrets() {
    let locker_dir = setup_test_environment();
    let test_secret_name = "test_list_secrets_secret";
    let test_file = locker_dir.join(format!("{}.slock", test_secret_name));

    // Ensure clean state
    cleanup_test_file(&format!("{}.slock", test_secret_name));

    // Create test file
    fs::write(&test_file, b"test").expect("Failed to create test file");

    // Get the list of secrets
    let secrets = list::list_secrets().expect("Failed to list secrets");

    // Verify our test secret is in the list
    assert!(
        secrets.contains(&"test_list_secrets_secret".to_string()),
        "Le fichier 'test_list_secrets_secret' n'apparaît pas dans la liste. Secrets : {:?}",
        secrets
    );

    // Clean up
    cleanup_test_file(&format!("{}.slock", test_secret_name));
}

#[test]
fn test_remove_secret() {
    let locker_dir = setup_test_environment();
    let test_secret_name = "test_remove_secret_secret";
    let test_file = locker_dir.join(format!("{}.slock", test_secret_name));

    // Create test file
    fs::write(&test_file, b"test").expect("Failed to create test file");
    assert!(test_file.exists(), "Test file was not created properly");

    // Remove the secret
    remove::remove_secret(test_secret_name).expect("Failed to remove secret");

    // Verify it's gone
    assert!(!test_file.exists(), "Secret file wasn't removed");
}

#[test]
fn test_encrypt_with_stdin() {
    use std::io::Cursor;
    let locker_dir = setup_test_environment();

    let secret_name = "test_encrypt_with_stdin_secret";
    let secret_value = "Ceci est un test";
    let encrypted_file = locker_dir.join(format!("{}.slock", secret_name));

    // Ensure clean state
    cleanup_test_file(&format!("{}.slock", secret_name));

    // Simulate stdin with a Cursor
    let mut stdin_mock = Cursor::new(secret_value);
    let mut input = String::new();
    stdin_mock
        .read_to_string(&mut input)
        .expect("Failed to read from mock stdin");

    // Encrypt using simulated input
    encrypt::encrypt(&input, secret_name).expect("Failed to encrypt from stdin");

    // Verify file was created
    assert!(
        encrypted_file.exists(),
        "Encrypted file wasn't created: {:?}",
        encrypted_file
    );

    // Clean up
    cleanup_test_file(&format!("{}.slock", secret_name));
}

#[test]
fn test_decrypt_with_stdout() {
    let locker_dir = setup_test_environment();

    let secret_name = "test_decrypt_with_stdout_secret";
    let secret_value = "Ceci est un test";
    let _encrypted_file = locker_dir.join(format!("{}.slock", secret_name));

    // Ensure clean state
    cleanup_test_file(&format!("{}.slock", secret_name));

    // Encrypt the secret
    encrypt::encrypt(secret_value, secret_name).expect("Failed to encrypt secret");

    // Decrypt and verify
    let decrypted_value = decrypt::decrypt(secret_name).expect("Failed to decrypt secret");
    assert_eq!(
        decrypted_value, secret_value,
        "Decrypted value doesn't match original"
    );

    // Clean up
    cleanup_test_file(&format!("{}.slock", secret_name));
}

#[cfg(not(feature = "disable_clipboard_tests"))]
#[test]
fn test_decrypt_with_clipboard() {
    use copypasta::{ClipboardContext, ClipboardProvider};

    // Skip test if environment variable is set
    if std::env::var("DISABLE_CLIPBOARD_TESTS").is_ok() {
        eprintln!("🛑 Clipboard test skipped via env var");
        return;
    }

    let locker_dir = setup_test_environment();

    let secret_name = "test_decrypt_with_clipboard_secret";
    let secret_value = "Ceci est un test";
    let _encrypted_file = locker_dir.join(format!("{}.slock", secret_name));

    // Ensure clean state
    cleanup_test_file(&format!("{}.slock", secret_name));

    // Encrypt the secret
    encrypt::encrypt(secret_value, secret_name).expect("Failed to encrypt secret");

    // Decrypt and verify clipboard
    let decrypted_value = decrypt::decrypt(secret_name).expect("Failed to decrypt secret");

    // Test clipboard functionality
    let mut ctx = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Unable to access clipboard, skipping test: {}", e);
            cleanup_test_file(&format!("{}.slock", secret_name));
            return;
        }
    };

    if let Err(e) = ctx.set_contents(decrypted_value.clone()) {
        eprintln!("Failed to set clipboard contents: {}", e);
        cleanup_test_file(&format!("{}.slock", secret_name));
        return;
    }

    match ctx.get_contents() {
        Ok(clipboard_content) => {
            assert_eq!(
                clipboard_content, secret_value,
                "Clipboard content doesn't match original"
            );
        }
        Err(e) => {
            eprintln!("Failed to get clipboard contents: {}", e);
        }
    }

    // Clean up
    cleanup_test_file(&format!("{}.slock", secret_name));
}
