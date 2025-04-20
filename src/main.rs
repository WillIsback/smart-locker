// Import necessary modules
use clap::{Arg, Command};
use colored::*; // For colored output
use smart_locker::commands::{
    decrypt::decrypt, encrypt::encrypt, export::export, list::list_secrets,
    migrate::migrate_metadata, remove::remove_secret, renew::renew_secret,
};
use smart_locker::utils::toolbox::{
    backup_key, copy_to_clipboard, init_locker_with_passphrase, restore_key,
};
use std::io::Read;
use std::process::exit;

fn main() {
    // Display the logo only for general help
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        display_logo();
    }
    // CLI command management
    let matches = Command::new("SmartLocker")
        .version(env!("CARGO_PKG_VERSION")) // Dynamically fetch version from Cargo.toml
        .author("William")
        .about("🔐 A CLI tool to encrypt and manage sensitive secrets")
        .long_about(
            "SmartLocker is a secret management tool that allows you to securely encrypt, \
        decrypt, list, and delete sensitive secrets.\n\n\
        Available commands:\n\
        - init: Initializes the vault and generates a symmetric key.\n\
            --passphrase: Passphrase to generate the symmetric key.\n\
        - encrypt: Encrypts a secret and saves it in the vault.\n\
            --name: Name of the secret.\n\
            --value: Value of the secret to encrypt.\n\
            --tags: Comma-separated tags for the secret (e.g., tag1,tag2).\n\
            --expiration: Expiration date in days (default: 15).\n\
            If --value is not provided, the value will be read from stdin.\n\n\
        - decrypt: Decrypts a secret.\n\
            --name: Name of the secret to decrypt.\n\
            --clipboard: Copies the decrypted secret to the clipboard.\n\n\
        - list: Lists all available secrets.\n\
        - remove: Deletes a secret.\n\
        - renew: Renews an expired secret.\n\
            --name: Name of the secret to renew.\n\
            --days: Number of additional days to extend the expiration (default: 15).\n\n\
        - backup-key: Creates a backup of the encryption key.\n\
        - restore-key: Restores the encryption key from a backup.\n\n\
        - export: Exports secrets to a file in a specified format.\n\
            --format: Format to export secrets (e.g., env).\n\
            --output: Output file path (default: .env).\n\n\
        Use --help or -h after a command for more details.",
        )
        .subcommand(
            Command::new("backup-key")
                .about("Creates a backup of the encryption key")
                .long_about(
                    "Creates a backup of the encryption key used to encrypt and decrypt secrets.\n\n\
                EXAMPLES:\n\
                - Backup the encryption key:\n\
                  smart-locker backup-key",
                ),
        )
        .subcommand(
            Command::new("restore-key")
                .about("Restores the encryption key from a backup")
                .long_about(
                    "Restores the encryption key from a previously created backup.\n\n\
                EXAMPLES:\n\
                - Restore the encryption key:\n\
                  smart-locker restore-key",
                ),
        )
        .subcommand(
            Command::new("init")
                .about("Initializes the vault and generates a symmetric key")
                .long_about(
                    "Initializes the vault by generating a symmetric key.\n\n\
                EXAMPLES:\n\
                - Generate a random key:\n\
                  smart-locker init\n\
                - Generate a key from a passphrase:\n\
                  smart-locker init --passphrase \"my passphrase\"",
                )
                .arg(
                    Arg::new("passphrase")
                        .short('p')
                        .long("passphrase")
                        .num_args(1)
                        .required(false)
                        .help("Passphrase to generate the symmetric key"),
                ),
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a secret")
                .long_about(
                    "Encrypts a secret and saves it in the vault.\n\n\
                EXAMPLES:\n\
                - Encrypt a secret with a value:\n\
                  smart-locker encrypt -n my_secret -v \"my value\"\n\
                - Encrypt a secret with tags:\n\
                  smart-locker encrypt -n my_secret -v \"my value\" --tags \"tag1,tag2\"\n\
                - Encrypt a secret by reading the value from stdin:\n\
                  echo \"my value\" | smart-locker encrypt -n my_secret,
                - Encrypt a secret with an expiration of 30 days:\n\
                  smart-locker encrypt -n my_secret -v \"my value\" --expiration 30\n"
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(true)
                        .help("Name of the secret"),
                )
                .arg(
                    Arg::new("value")
                        .short('v')
                        .long("value")
                        .num_args(1)
                        .required(false)
                        .help("Value of the secret to encrypt"),
                )
                .arg(
                    Arg::new("tags")
                        .short('t')
                        .long("tags")
                        .num_args(1)
                        .required(false)
                        .help("Comma-separated tags for the secret (e.g., tag1,tag2)"),
                )
                .arg(
                    Arg::new("expiration")
                        .short('e')
                        .long("expiration")
                        .num_args(1)
                        .required(false)
                        .default_value("15")
                        .help("Expiration date in days (default: 15)"),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a secret")
                .long_about(
                    "Decrypts a secret and displays its value or copies it to the clipboard.\n\n\
                EXAMPLES:\n\
                - Decrypt a secret and display it:\n\
                  smart-locker decrypt -n my_secret\n\
                - Decrypt a secret and copy it to the clipboard:\n\
                  smart-locker decrypt -n my_secret --clipboard",
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(true)
                        .help("Name of the secret to decrypt"),
                )
                .arg(
                    Arg::new("clipboard")
                        .short('c')
                        .long("clipboard")
                        .action(clap::ArgAction::SetTrue)
                        .required(false)
                        .help("Copies the decrypted secret to the clipboard"),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Lists all available secrets")
                .long_about(
                    "Displays the list of available secrets in the vault.\n\n\
                EXAMPLES:\n\
                - List all secrets:\n\
                  smart-locker list",
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Deletes a secret")
                .long_about(
                    "Deletes a secret from the vault.\n\n\
                EXAMPLES:\n\
                - Delete a secret:\n\
                  smart-locker remove -n my_secret",
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(true)
                        .help("Name of the secret to delete"),
                ),
        )
        .subcommand(
            Command::new("export")
                .about("Exports secrets to a file in a specified format")
                .long_about(
                    "Exports secrets to a file in a specified format. Supported formats include:
                    - env: Exports secrets as environment variables in a .env file.

                EXAMPLES:
                - Export secrets to a .env file:
                  smart-locker export --format env --output .env",
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .num_args(1)
                        .required(true)
                        .help("Format to export secrets (e.g., env)"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .num_args(1)
                        .required(false)
                        .help("Output file path (default: .env)"),
                ),
        )
        .subcommand(
            Command::new("renew")
                .about("Renews an expired secret")
                .long_about(
                    "Renews an expired secret by extending its expiration date.\n\n\
                EXAMPLES:\n\
                - Renew an expired secret:\n\
                  smart-locker renew -n my_secret -d 30\n\
                - Renew an expired secret with a default expiration of 15 days:\n\
                  smart-locker renew -n my_secret",
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(true)
                        .help("Name of the secret to renew"),
                )
                .arg(
                    Arg::new("days")
                        .short('d')
                        .long("days")
                        .num_args(1)
                        .required(false)
                        .default_value("15")
                        .help("Number of additional days to extend the expiration"),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        display_logo(); // Affiche le logo uniquement pour la commande init

        // Récupérer la passphrase si elle est fournie
        let passphrase = matches.get_one::<String>("passphrase").map(|s| s.as_str());

        // Appeler init_locker_with_passphrase avec ou sans passphrase
        init_locker_with_passphrase(passphrase).expect("Failed to initialize the vault");

        println!("{}", "✅ Vault initialized successfully!".green());
    } else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let name = matches.get_one::<String>("name").expect("Name is required");
        let value = if let Some(value) = matches.get_one::<String>("value") {
            value.clone()
        } else {
            // Read from stdin if --value is not provided
            let mut input = String::new();
            match std::io::stdin().read_to_string(&mut input) {
                Ok(_) => input.trim().to_string(),
                Err(err) => {
                    eprintln!("{}", format!("Error reading from stdin: {}", err).red());
                    exit(1);
                }
            }
        };

        let tags = matches
            .get_one::<String>("tags")
            .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(Vec::new);

        let expiration: u64 = matches
            .get_one::<String>("expiration")
            .map(|s| {
                s.trim().parse().unwrap_or_else(|_| {
                    eprintln!(
                        "{}",
                        "Invalid expiration value, using default of 15 days".red()
                    );
                    15
                })
            })
            .unwrap_or(15);

        // Encrypt the secret
        match encrypt(&value, name, tags, Some(expiration)) {
            Ok(_) => println!(
                "{}",
                format!(
                    "✅ Secret '{}' encrypted successfully with expiration of {} days!",
                    name, expiration
                )
                .green()
            ),
            Err(err) => {
                eprintln!("{}", format!("Error encrypting secret: {}", err).red());
                exit(1);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let name = matches.get_one::<String>("name").expect("Name is required");

        match decrypt(name) {
            Ok(decrypted_value) => {
                println!("DEBUG: Decrypted value: {}", decrypted_value); // Log temporaire
                if matches.get_flag("clipboard") {
                    if let Err(err) = copy_to_clipboard(&decrypted_value) {
                        eprintln!(
                            "{}",
                            format!("Error copying to the clipboard: {}", err).red()
                        );
                        exit(1);
                    }
                    println!("{}", "✅ Secret copied to the clipboard!".green());
                } else {
                    println!("{}", decrypted_value.green());
                }
            }
            Err(err) => {
                eprintln!("{}", format!("Error decrypting secret: {}", err).red());
                exit(1);
            }
        }
    } else if matches.subcommand_matches("list").is_some() {
        match list_secrets() {
            Ok(secrets) => {
                if secrets.is_empty() {
                    println!("{}", "⚠️ No secrets found.".yellow());
                } else {
                    println!("{}", "🔒 Available secrets:".blue());
                    for secret in secrets {
                        println!("{}", secret);
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", format!("Error listing secrets: {}", err).red());
                exit(1);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.get_one::<String>("name").unwrap();
        remove_secret(name).expect("Failed to delete the secret");
        println!(
            "{}",
            format!("✅ Secret '{}' deleted successfully!", name).green()
        );
    } else if let Some(matches) = matches.subcommand_matches("renew") {
        let name = matches.get_one::<String>("name").unwrap();
        let days: u64 = matches
            .get_one::<String>("days")
            .unwrap()
            .parse()
            .expect("Invalid number of days");

        match renew_secret(name, days) {
            Ok(_) => println!("✅ Secret '{}' renewed successfully!", name),
            Err(err) => {
                eprintln!("{}", format!("Error renewing secret: {}", err).red());
                exit(1);
            }
        }
    } else if matches.subcommand_matches("backup-key").is_some() {
        backup_key().expect("Failed to create a backup of the encryption key");
        println!("{}", "✅ Encryption key backed up successfully!".green());
    } else if matches.subcommand_matches("restore-key").is_some() {
        restore_key().expect("Failed to restore the encryption key");
        println!("{}", "✅ Encryption key restored successfully!".green());
    } else if let Some(matches) = matches.subcommand_matches("export") {
        let format = matches.get_one::<String>("format").unwrap();
        let output = matches.get_one::<String>("output").map(|s| s.as_str());
        export(format, output).expect("Failed to export secrets");
    }
}

fn display_logo() {
    println!(
        "{}",
        "🦀🔐 SmartLocker - Secure your secrets with Rust!"
            .bold()
            .green()
    );
}
