use crate::LockerResult;
use crate::SmartLockerError;
use colored::Colorize;
use copypasta::{ClipboardContext, ClipboardProvider};
use directories::UserDirs;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Vérifie et crée un répertoire s'il n'existe pas.
///
/// # Arguments
///
/// * `path` - Le chemin du répertoire à vérifier ou à créer.
///
/// # Retourne
///
/// * `LockerResult<()>` - Un résultat indiquant si l'opération a réussi ou non.
///
/// # Exemple
///
/// ```rust
/// use std::path::PathBuf;
/// use smart_locker::utils::toolbox::ensure_dir_exists;
///
/// let path = PathBuf::from("/tmp/my_locker");
/// ensure_dir_exists(&path).expect("Failed to create directory");
/// ```
///
/// Ce code crée le répertoire `/tmp/my_locker` s'il n'existe pas déjà.
///
/// # Notes
///
/// Si le répertoire existe déjà, la fonction ne fait rien.
/// Si une erreur se produit, elle retourne un `SmartLockerError`.
pub fn ensure_dir_exists(path: &PathBuf) -> LockerResult<()> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| {
            SmartLockerError::FileSystemError(format!("Error creating folder {:?}: {}", path, e))
        })?;
        println!("✅ Directory created: {:?}", path);
    }
    Ok(())
}

/// Retourne le chemin du répertoire `.locker`.
///
/// # Retourne
///
/// * `LockerResult<PathBuf>` - Le chemin du répertoire `.locker`.
///
/// # Exemple
///
/// ```rust
/// use smart_locker::utils::toolbox::get_locker_dir;
///
/// let locker_dir = get_locker_dir().expect("Failed to get locker directory");
/// println!("Locker directory: {:?}", locker_dir);
/// ```
///
/// Ce code retourne le chemin du répertoire `.locker`, qui est soit défini par une variable d'environnement, soit par défaut dans le répertoire utilisateur.
///
/// # Notes
///
/// Si une variable d'environnement `SMART_LOCKER_TEST_DIR` ou `SMART_LOCKER_TEST_DIR_*` est définie, elle sera utilisée comme chemin.
/// Sinon, le répertoire par défaut est `~/.locker`.
pub fn get_locker_dir() -> LockerResult<PathBuf> {
    // Rechercher une variable d'environnement spécifique au test
    if let Some((_, value)) = env::vars()
        .filter(|(key, _)| key.starts_with("SMART_LOCKER_TEST_DIR_"))
        .last()
    // Prend la dernière variable définie
    {
        return Ok(PathBuf::from(value));
    }

    // Si aucune variable spécifique n'est trouvée, utiliser le répertoire par défaut
    if let Ok(test_dir) = env::var("SMART_LOCKER_TEST_DIR") {
        Ok(PathBuf::from(test_dir))
    } else {
        let user_dirs = UserDirs::new().ok_or_else(|| {
            SmartLockerError::FileSystemError("Unable to access user directory".to_string())
        })?;
        Ok(user_dirs.home_dir().join(".locker"))
    }
}
/// Vérifie si le fichier donné est un secret valide avec l'extension `.slock`.
///
/// # Arguments
///
/// * `file_path` - Le chemin du fichier à vérifier.
/// * `silent` - Si `true`, la fonction n'affiche aucun message.
///
/// # Retourne
///
/// * `(bool, Option<String>)` - Un tuple où :
///   - `bool` indique si le fichier est un secret valide.
///   - `Option<String>` contient le nom du secret (`secret_name`) si le fichier est valide.
///
/// # Exemple
///
/// ```rust
/// use std::path::PathBuf;
/// use smart_locker::utils::toolbox::is_this_secret;
///
/// let file_path = PathBuf::from("example.slock");
/// let (is_valid, secret_name) = is_this_secret(&file_path, false);
/// if is_valid {
///     println!("Secret valid: {}", secret_name.unwrap());
/// } else {
///     println!("Invalid secret file.");
/// }
/// ```
///
/// Ce code vérifie si `example.slock` est un fichier secret valide et affiche son nom si c'est le cas.
///
/// # Notes
///
/// Si le fichier n'est pas valide et que `silent` est `false`, un message d'avertissement est affiché.
pub fn is_this_secret(file_path: &Path, silent: bool) -> (bool, Option<String>) {
    if file_path.extension().and_then(|ext| ext.to_str()) == Some("slock") {
        if let Some(secret_name) = file_path.file_stem().and_then(|stem| stem.to_str()) {
            return (true, Some(secret_name.to_string()));
        } else if !silent {
            println!(
                "{}",
                format!("⚠️ Invalid secret file name '{}'.", file_path.display()).yellow()
            );
        }
        (false, None)
    } else {
        if !silent {
            println!(
                "{}",
                format!(
                    "⚠️ The file '{}' is not a valid secret file.",
                    file_path.display()
                )
                .yellow()
            );
        }
        (false, None)
    }
}


/// Copie une chaîne de caractères dans le presse-papiers.
///
/// # Arguments
///
/// * `content` - La chaîne de caractères à copier dans le presse-papiers.
///
/// # Retourne
///
/// * `Result<(), String>` - Un résultat indiquant si l'opération a réussi ou non.
///
/// # Notes
///
/// Cette fonction est compatible avec Windows, Linux, et WSL.
/// Si l'accès au presse-papiers échoue, une erreur est retournée avec un message explicatif.
pub fn copy_to_clipboard(content: &str) -> Result<(), String> {
    println!("Attempting to copy to clipboard...");

    if cfg!(target_os = "windows") {
        // Utilisation de copypasta pour Windows
        let mut ctx = ClipboardContext::new()
            .map_err(|_| "Unable to access the clipboard on Windows".to_string())?;
        ctx.set_contents(content.to_string())
            .map_err(|_| "Failed to copy content to the clipboard on Windows".to_string())?;
        println!("✅ Content copied to clipboard on Windows.");
        Ok(())
    } else if cfg!(target_os = "linux") {
        // Vérification si on est sous WSL
        if is_wsl() {
            // Utilisation de clip.exe pour WSL
            Command::new("clip.exe")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    if let Some(stdin) = child.stdin.as_mut() {
                        use std::io::Write;
                        stdin.write_all(content.as_bytes())?;
                    }
                    child.wait() // Attendre la fin du processus
                })
                .map_err(|_| "Failed to copy content to clipboard on WSL".to_string())?;
            println!("✅ Content copied to clipboard on WSL.");
            Ok(())
        } else {
            // Utilisation de xclip ou xsel pour Linux natif
            if Command::new("xclip").output().is_ok() {
                Command::new("xclip")
                    .arg("-selection")
                    .arg("clipboard")
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        if let Some(stdin) = child.stdin.as_mut() {
                            use std::io::Write;
                            stdin.write_all(content.as_bytes())?;
                        }
                        child.wait()
                    })
                    .map_err(|_| "Failed to copy content to clipboard using xclip".to_string())?;
                println!("✅ Content copied to clipboard using xclip.");
                Ok(())
            } else if Command::new("xsel").output().is_ok() {
                Command::new("xsel")
                    .arg("--clipboard")
                    .arg("--input")
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        if let Some(stdin) = child.stdin.as_mut() {
                            use std::io::Write;
                            stdin.write_all(content.as_bytes())?;
                        }
                        child.wait()
                    })
                    .map_err(|_| "Failed to copy content to clipboard using xsel".to_string())?;
                println!("✅ Content copied to clipboard using xsel.");
                Ok(())
            } else {
                Err("No clipboard utility (xclip or xsel) found on Linux".to_string())
            }
        }
    } else {
        Err("Unsupported operating system".to_string())
    }
}

/// Vérifie si le programme tourne sous WSL.
///
/// # Retourne
///
/// * `bool` - `true` si le programme tourne sous WSL, sinon `false`.
fn is_wsl() -> bool {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            return stdout.to_lowercase().contains("microsoft");
        }
    }
    false
}