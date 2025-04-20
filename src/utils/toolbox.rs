use crate::LockerResult;
use crate::SmartLockerError;
use colored::Colorize;
use copypasta::{ClipboardContext, ClipboardProvider};
use directories::UserDirs;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Vérifie et crée un répertoire s'il n'existe pas.
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
/// let file_path = PathBuf::from("example.slock");
/// let (is_valid, secret_name) = is_this_secret(&file_path, false);
/// if is_valid {
///     println!("Secret valid: {}", secret_name.unwrap());
/// }
/// ```
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
/// Retourne une erreur si l'opération échoue.
pub fn copy_to_clipboard(content: &str) -> Result<(), String> {
    let mut ctx =
        ClipboardContext::new().map_err(|_| "Unable to access the clipboard".to_string())?;
    ctx.set_contents(content.to_string())
        .map_err(|_| "Failed to copy content to the clipboard".to_string())
}
