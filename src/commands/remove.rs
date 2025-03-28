use std::fs;
use directories::UserDirs;

pub fn remove_secret(name: &str) {
    // Get the path of the secure folder
    let user_dirs = UserDirs::new().expect("Unable to access user directory");
    let locker_dir = user_dirs.home_dir().join(".locker");

    // Check if the secure folder exists
    if !locker_dir.exists() {
        println!("No secure folder found. Run `init` to create it.");
        return;
    }

    // Build the full path of the file to delete
    let file_path = locker_dir.join(format!("{}.slock", name));

    // Check if the file exists
    if file_path.exists() {
        // Delete the file
        fs::remove_file(&file_path).expect("Error when deleting the file");
        println!("Secret '{}' has been successfully deleted.", name);
    } else {
        println!("Secret '{}' doesn't exist.", name);
    }
}