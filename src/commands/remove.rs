use std::fs;
use directories::UserDirs;

pub fn remove_secret(name: &str) {
    // Obtenir le chemin du dossier sécurisé
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    // Vérifier si le dossier sécurisé existe
    if !locker_dir.exists() {
        println!("Aucun dossier sécurisé trouvé. Exécutez `init` pour le créer.");
        return;
    }

    // Construire le chemin complet du fichier à supprimer
    let file_path = locker_dir.join(format!("{}.slock", name));

    // Vérifier si le fichier existe
    if file_path.exists() {
        // Supprimer le fichier
        fs::remove_file(&file_path).expect("Erreur lors de la suppression du fichier");
        println!("Le secret '{}' a été supprimé avec succès.", name);
    } else {
        println!("Le secret '{}' n'existe pas.", name);
    }
}