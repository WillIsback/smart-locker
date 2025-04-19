use crate::commands::list::list_secrets;
use std::env;
use std::fs;
use std::path::PathBuf;

pub struct ExportFormat;

impl ExportFormat {
    pub fn export_env_with_placeholders(secret_names: &Vec<String>, output_path: &PathBuf) {
        let mut content = String::new();
        for secret_name in secret_names {
            content.push_str(&format!(
                "{}=$(smart-locker decrypt -n {})\n",
                secret_name, secret_name
            ));
        }
        fs::write(output_path, content).expect("Erreur lors de l'écriture du fichier .env");
    }
}

pub fn export(format: &str, output_file: Option<&str>) {
    let current_dir: PathBuf =
        env::current_dir().expect("Impossible de récupérer le répertoire courant");

    // Construire le chemin de sortie
    let output_path = match output_file {
        Some(file) => current_dir.join(file),
        None => current_dir.join(".env"), // Valeur par défaut si aucun fichier n'est spécifié
    };

    // Liste des secrets à exporter
    let secret_list: Vec<String> = list_secrets();
    if secret_list.is_empty() {
        eprintln!("Aucun secret à exporter.");
        return;
    }

    match format {
        "env" => ExportFormat::export_env_with_placeholders(&secret_list, &output_path),
        _ => eprintln!("Format non supporté : {}", format),
    }

    println!("Le fichier a été exporté à : {:?}", output_path);
}
