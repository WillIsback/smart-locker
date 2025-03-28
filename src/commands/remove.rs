use std::fs;
use crate::utils::toolbox::get_locker_dir;

pub fn remove_secret(name: &str) {
    let locker_dir = get_locker_dir();

    if !locker_dir.exists() {
        println!("No secure folder found. Run `init` to create it.");
        return;
    }

    let file_path = locker_dir.join(format!("{}.slock", name));
    if file_path.exists() {
        fs::remove_file(&file_path).expect("Error when deleting the file");
        println!("Secret '{}' has been successfully deleted.", name);
    } else {
        println!("Secret '{}' doesn't exist.", name);
    }
}