use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub type LockerResult<T> = Result<T, SmartLockerError>;

pub mod commands;
pub use crate::commands::{
    decrypt::decrypt,
    encrypt::encrypt,
    export::export,
    init::{backup_key, init_locker_with_passphrase, restore_key},
    list::list_secrets,
    remove::remove_secret,
    renew::renew_secret,
};
pub mod utils;

pub use crate::utils::config::EncryptionConfig;

pub use crate::utils::toolbox::{copy_to_clipboard, get_locker_dir};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecretMetadata {
    name: String,
    created_at: u64,
    expire_at: u64,
    expired: bool,
    tags: Vec<String>,
}

impl SecretMetadata {
    pub fn field_count(instance: Option<&Self>) -> usize {
        // Si une instance est fournie, sérialiser cette instance
        let json_value = if let Some(instance) = instance {
            serde_json::to_value(instance).expect("Failed to serialize instance")
        } else {
            // Sinon, sérialiser une instance par défaut de `SecretMetadata`
            serde_json::to_value(SecretMetadata {
                name: String::new(),
                created_at: 0,
                expire_at: 0,
                expired: false,
                tags: Vec::new(),
            })
            .expect("Failed to serialize default instance")
        };

        // Compter les clés dans l'objet JSON
        if let Value::Object(map) = json_value {
            map.len() // Retourner le nombre de clés
        } else {
            0 // Fallback en cas d'échec
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataFile {
    pub secrets: HashMap<String, SecretMetadata>, // Clé : nom du secret
}
