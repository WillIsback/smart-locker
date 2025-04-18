// Import necessary modules
use clap::{Arg, Command};
use colored::*; // For colored output
use smart_locker::commands::{
    decrypt::decrypt, encrypt::encrypt, list::list_secrets, remove::remove_secret,
};

use smart_locker::utils::toolbox::{derive_key_from_passphrase, get_locker_dir, init_locker};
use std::fs;
use std::io::Read;
use std::process::exit;

fn main() {
    // Display the logo only for general help
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        display_logo();
    }

    // Check if the ~/.locker folder exists
    let locker_dir = match get_locker_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!(
                "{}",
                format!("Error getting locker directory: {}", err).red()
            );
            exit(1);
        }
    };

    // CLI command management
    let matches = Command::new("SmartLocker")
        .version("1.0")
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
            If --value is not provided, the value will be read from stdin.\n\n\
        - decrypt: Decrypts a secret.\n\
            --name: Name of the secret to decrypt.\n\
            --clipboard: Copies the decrypted secret to the clipboard.\n\n\
        - list: Lists all available secrets.\n\
        - remove: Deletes a secret.\n\n\
        Use --help or -h after a command for more details.",
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
                - Encrypt a secret by reading the value from stdin:\n\
                  echo \"my value\" | smart-locker encrypt -n my_secret",
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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        display_logo(); // Display the logo only for the init command
        if let Some(passphrase) = matches.get_one::<String>("passphrase") {
            let salt = b"smartlocker_salt"; // You can customize the salt
            let key = match derive_key_from_passphrase(passphrase, salt) {
                Ok(k) => k,
                Err(err) => {
                    eprintln!("{}", format!("Error deriving key: {}", err).red());
                    exit(1);
                }
            };

            let key_path = locker_dir.join("locker.key");
            if let Err(err) = fs::write(&key_path, key) {
                eprintln!("{}", format!("Error writing the key: {}", err).red());
                exit(1);
            }
            println!(
                "{}",
                format!(
                    "✅ Key generated from the passphrase and saved: {:?}",
                    key_path
                )
                .green()
            );
        } else {
            if let Err(err) = init_locker() {
                eprintln!("{}", format!("Error initializing locker: {}", err).red());
                exit(1);
            }
            println!("{}", "✅ Vault initialized successfully!".green());
        }
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

        match encrypt(&value, name) {
            Ok(_) => println!(
                "{}",
                format!("✅ Secret '{}' encrypted successfully!", name).green()
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
                if matches.get_flag("clipboard") {
                    if cfg!(target_os = "linux") && std::env::var("WSL_DISTRO_NAME").is_ok() {
                        use std::process::{Command, Stdio};
                        match Command::new("clip.exe").stdin(Stdio::piped()).spawn() {
                            Ok(mut child) => {
                                {
                                    let stdin = match child.stdin.as_mut() {
                                        Some(stdin) => stdin,
                                        None => {
                                            eprintln!("{}", "Unable to access stdin".red());
                                            exit(1);
                                        }
                                    };
                                    use std::io::Write;
                                    if let Err(err) = stdin.write_all(decrypted_value.as_bytes()) {
                                        eprintln!(
                                            "{}",
                                            format!("Error writing to clip.exe: {}", err).red()
                                        );
                                        exit(1);
                                    }
                                }
                                if let Err(err) = child.wait() {
                                    eprintln!(
                                        "{}",
                                        format!("Error executing clip.exe: {}", err).red()
                                    );
                                    exit(1);
                                }
                                println!("{}", "✅ Secret copied to Windows clipboard!".green());
                            }
                            Err(err) => {
                                eprintln!(
                                    "{}",
                                    format!("Unable to execute clip.exe: {}", err).red()
                                );
                                exit(1);
                            }
                        };
                    } else {
                        // Copy to Linux clipboard
                        use copypasta::{ClipboardContext, ClipboardProvider};
                        let mut ctx =
                            ClipboardContext::new().expect("Unable to access the clipboard");
                        if let Err(err) = ctx.set_contents(decrypted_value.clone()) {
                            eprintln!(
                                "{}",
                                format!("Error copying to the clipboard: {}", err).red()
                            );
                            exit(1);
                        }
                        println!("{}", "✅ Secret copied to the clipboard!".green());
                    }
                } else {
                    // Print the decrypted value to the terminal
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
        let name = matches.get_one::<String>("name").expect("Name is required");
        match remove_secret(name) {
            Ok(_) => println!(
                "{}",
                format!("✅ Secret '{}' deleted successfully!", name).green()
            ),
            Err(err) => {
                eprintln!("{}", format!("Error deleting secret: {}", err).red());
                exit(1);
            }
        }
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
