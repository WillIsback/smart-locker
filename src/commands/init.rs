fn initialize_locker() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::PathBuf;
    use rand::Rng;

    // Define the path for the locker key
    let locker_key_path = PathBuf::from("~/.locker/locker.key").expand()?;

    // Generate a random encryption key
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen(); // AES-256 requires a 32-byte key

    // Create the locker directory if it doesn't exist
    if let Some(parent) = locker_key_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the key to the locker.key file
    fs::write(locker_key_path, &key)?;

    println!("Locker initialized. Key saved to {:?}", locker_key_path);
    Ok(())
}