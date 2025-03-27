mod commands;
// Importation des modules nécessaires
use std::fs;
use std::io::Read;
use directories::UserDirs;
use clap::{Arg, Command};
use commands::encrypt::encrypt;
use commands::decrypt::decrypt;
use commands::list::list_secrets;
use commands::remove::remove_secret;
use smart_locker::utils::init_locker;
use smart_locker::utils::derive_key_from_passphrase;
use colored::*; // Pour les couleurs
use std::env;

fn main() {
    // Afficher le logo uniquement pour l'aide générale
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        display_logo();
    }
    println!("Current working directory: {:?}", env::current_dir().unwrap());
    // Vérifier si le dossier ~/.locker existe
    let user_dirs = UserDirs::new().expect("Impossible d'accéder au dossier utilisateur");
    let locker_dir = user_dirs.home_dir().join(".locker");

    // Gestion des commandes CLI
    let matches = Command::new("SmartLocker")
    .version("1.0")
    .author("William")
    .about("🔐 Un outil CLI pour chiffrer et gérer des secrets sensibles")
    .long_about(
        "SmartLocker est un outil de gestion de secrets qui vous permet de chiffrer, \
        déchiffrer, lister et supprimer des secrets sensibles en toute sécurité.\n\n\
        Commandes disponibles :\n\
        - init : Initialise le coffre-fort et génère une clé symétrique.\n\
            --passphrase : Passphrase pour générer la clé symétrique.\n\
        - encrypt : Chiffre un secret et le sauvegarde dans le coffre-fort.\n\
            --name : Nom du secret.\n\
            --value : Valeur du secret à chiffrer.\n\
            Si --value n'est pas fourni, la valeur sera lue depuis stdin.\n\n\
        - decrypt : Déchiffre un secret.\n\
            --name : Nom du secret à déchiffrer.\n\
            --clipboard : Copie le secret déchiffré dans le presse-papier.\n\n\
        - list : Liste tous les secrets disponibles.\n\
        - remove : Supprime un secret.\n\n\
        Utilisez --help ou -h après une commande pour plus de détails.",
    )
    .subcommand(
        Command::new("init")
            .about("Initialise le coffre-fort et génère une clé symétrique")
            .long_about(
                "Initialise le coffre-fort en générant une clé symétrique.\n\n\
                EXEMPLES :\n\
                - Générer une clé aléatoire :\n\
                  smart-locker init\n\
                - Générer une clé à partir d'une passphrase :\n\
                  smart-locker init --passphrase \"ma passphrase\"",
            )
            .arg(
                Arg::new("passphrase")
                    .short('p')
                    .long("passphrase")
                    .num_args(1)
                    .required(false)
                    .help("Passphrase pour générer la clé symétrique"),
            ),
    )
    .subcommand(
        Command::new("encrypt")
            .about("Chiffre un secret")
            .long_about(
                "Chiffre un secret et le sauvegarde dans le coffre-fort.\n\n\
                EXEMPLES :\n\
                - Chiffrer un secret avec une valeur :\n\
                  smart-locker encrypt -n my_secret -v \"ma valeur\"\n\
                - Chiffrer un secret en lisant la valeur depuis stdin :\n\
                  echo \"ma valeur\" | smart-locker encrypt -n my_secret",
            )
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .num_args(1)
                .required(true)
                .help("Nom du secret"))
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .num_args(1)
                .required(false)
                .help("Valeur du secret à chiffrer")),
    )
    .subcommand(
        Command::new("decrypt")
            .about("Déchiffre un secret")
            .long_about(
                "Déchiffre un secret et affiche sa valeur ou la copie dans le presse-papier.\n\n\
                EXEMPLES :\n\
                - Déchiffrer un secret et l'afficher :\n\
                  smart-locker decrypt -n my_secret\n\
                - Déchiffrer un secret et le copier dans le presse-papier :\n\
                  smart-locker decrypt -n my_secret --clipboard",
            )
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .num_args(1)
                .required(true)
                .help("Nom du secret à déchiffrer"))
            .arg(Arg::new("clipboard")
                .short('c')
                .long("clipboard")
                .action(clap::ArgAction::SetTrue)
                .required(false)
                .help("Copie le secret déchiffré dans le presse-papier")),
    )
    .subcommand(
        Command::new("list")
            .about("Liste tous les secrets disponibles")
            .long_about(
                "Affiche la liste des secrets disponibles dans le coffre-fort.\n\n\
                EXEMPLES :\n\
                - Lister tous les secrets :\n\
                  smart-locker list",
            ),
    )
    .subcommand(
        Command::new("remove")
            .about("Supprime un secret")
            .long_about(
                "Supprime un secret du coffre-fort.\n\n\
                EXEMPLES :\n\
                - Supprimer un secret :\n\
                  smart-locker remove -n my_secret",
            )
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .num_args(1)
                .required(true)
                .help("Nom du secret à supprimer")),
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        display_logo(); // Afficher le logo uniquement pour la commande init
        if let Some(passphrase) = matches.get_one::<String>("passphrase") {
            let salt = b"smartlocker_salt"; // Vous pouvez personnaliser le sel
            let key = derive_key_from_passphrase(passphrase, salt);

            let key_path = locker_dir.join("locker.key");
            fs::write(&key_path, key).expect("Erreur lors de l'écriture de la clé");
            println!("{}", format!("✅ Clé générée à partir de la passphrase et sauvegardée : {:?}", key_path).green());
        } else {
            init_locker(); // Appeler la fonction existante pour générer une clé aléatoire
            println!("{}", "✅ Coffre-fort initialisé avec succès !".green());
        }
    } else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let name = matches.get_one::<String>("name").unwrap();
        let value = if let Some(value) = matches.get_one::<String>("value") {
            value.clone()
        } else {
            // Lire depuis stdin si --value n'est pas fourni
            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .expect("Erreur lors de la lecture de stdin");
            input.trim().to_string()
        };
        encrypt(&value, name);
        println!("{}", format!("✅ Secret '{}' chiffré avec succès !", name).green());
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let name = matches.get_one::<String>("name").unwrap();
        let decrypted_value = decrypt(name);
        if matches.get_flag("clipboard") {
            if cfg!(target_os = "linux") && std::env::var("WSL_DISTRO_NAME").is_ok() {
                // Utiliser clip.exe pour WSL
                use std::process::{Command, Stdio};
                let mut child = Command::new("clip.exe")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Impossible d'exécuter clip.exe");
                {
                    let stdin = child.stdin.as_mut().expect("Impossible d'accéder à stdin");
                    use std::io::Write;
                    stdin
                        .write_all(decrypted_value.as_bytes())
                        .expect("Erreur lors de l'écriture dans clip.exe");
                }
                child.wait().expect("Erreur lors de l'exécution de clip.exe");
                println!("{}", "✅ Secret copié dans le presse-papier Windows !".green());
            } else {
                // Copier dans le presse-papier Linux
                use copypasta::{ClipboardContext, ClipboardProvider};
                let mut ctx = ClipboardContext::new().expect("Impossible d'accéder au presse-papier");
                ctx.set_contents(decrypted_value.clone())
                    .expect("Erreur lors de la copie dans le presse-papier");
                println!("{}", "✅ Secret copié dans le presse-papier !".green());
            }
        }
    }else if matches.subcommand_matches("list").is_some() {
        let secrets = list_secrets(&locker_dir);
        if secrets.is_empty() {
            println!("{}", "⚠️ Aucun secret trouvé.".yellow());
        } else {
            println!("{}", "🔒 Secrets disponibles :".blue());
            for secret in secrets {
                println!("{}", secret);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.get_one::<String>("name").unwrap();
        remove_secret(name);
        println!("{}", format!("✅ Secret '{}' supprimé avec succès !", name).green());
    }
}

fn display_logo() {

        println!("{}", "🦀🔐 SmartLocker - Sécurisez vos secrets avec Rust !".bold().green());

}