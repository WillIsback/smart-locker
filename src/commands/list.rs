// This file implements the `list` command. It exports a function `list_secrets` that lists all stored secrets in the `~/.locker` directory.

use std::fs;
use std::path::PathBuf;

pub fn list_secrets() -> Result<(), Box<dyn std::error::Error>> {
    let locker_dir = dirs::home_dir().unwrap().join(".locker");
    
    if locker_dir.exists() && locker_dir.is_dir() {
        let entries = fs::read_dir(locker_dir)?;

        println!("Secrets stored in ~/.locker:");
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                println!("{}", path.display());
            }
        }
    } else {
        println!("No secrets found. The ~/.locker directory does not exist.");
    }

    Ok(())
}