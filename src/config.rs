// This file manages configuration settings for the SmartLocker application.

use std::path::PathBuf;

pub struct Config {
    pub locker_dir: PathBuf,
    pub key_file: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let locker_dir = home_dir.join(".locker");
        let key_file = locker_dir.join("locker.key");

        Self { locker_dir, key_file }
    }
}