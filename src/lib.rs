use thiserror::Error;

pub mod commands;
pub use crate::commands::{
    decrypt::decrypt, encrypt::encrypt, list::list_secrets, remove::remove_secret,
};
pub mod utils;
pub use crate::utils::toolbox::{derive_key_from_passphrase, get_locker_dir, init_locker};

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
