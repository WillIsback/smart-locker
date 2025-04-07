use crate::utils::toolbox::get_locker_dir;
use crate::SmartLockerError;
use std::fs;

pub fn remove_secret(name: &str) -> Result<(), SmartLockerError> {
    let locker_dir = get_locker_dir()?;

    if !locker_dir.exists() {
        println!("No secure folder found. Run `init` to create it.");
        return Ok(());
    }

    let file_path = locker_dir.join(format!("{}.slock", name));
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error when deleting the file: {}", e))
        })?;
        println!("Secret '{}' has been successfully deleted.", name);
    } else {
        println!("Secret '{}' doesn't exist.", name);
    }

    Ok(())
}
