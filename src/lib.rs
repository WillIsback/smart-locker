use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type LockerResult<T> = Result<T, SmartLockerError>;

pub mod commands;
pub use crate::commands::{
    decrypt::decrypt, encrypt::encrypt, export::export, list::list_secrets, remove::remove_secret,
    renew::renew_secret,
};
pub mod utils;
pub use crate::utils::toolbox::{
    backup_key, get_locker_dir, init_locker_with_passphrase, restore_key,
};

#[derive(Error, Debug)]
pub enum SmartLockerError {
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    #[error("Initialization error: {0}")]
    InitializationError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

#[derive(Serialize, Deserialize, Clone)]
struct SecretMetadata {
    name: String,
    created_at: u64,
    expire_at: u64,
    expired: bool,
    tags: Vec<String>,
}
