use thiserror::Error;

pub mod commands;
pub use crate::commands::{
    decrypt::decrypt, encrypt::encrypt, export::export, list::list_secrets, remove::remove_secret,
};
pub mod utils;
pub use crate::utils::toolbox::{
    backup_key, get_locker_dir, init_locker_with_passphrase, restore_key,
};
