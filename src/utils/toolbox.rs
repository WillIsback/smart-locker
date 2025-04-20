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
/// # Exemple
///
/// ```rust
/// use smart_locker::utils::toolbox::copy_to_clipboard;
///
/// let content = "This is a test";
/// copy_to_clipboard(content).expect("Failed to copy to clipboard");
/// ```
///
/// Ce code copie la chaîne "This is a test" dans le presse-papiers.
///
/// # Notes
///
/// Si l'accès au presse-papiers échoue, une erreur est retournée avec un message explicatif.
pub fn copy_to_clipboard(content: &str) -> Result<(), String> {
    let mut ctx =
        ClipboardContext::new().map_err(|_| "Unable to access the clipboard".to_string())?;
    ctx.set_contents(content.to_string())
        .map_err(|_| "Failed to copy content to the clipboard".to_string())
}
