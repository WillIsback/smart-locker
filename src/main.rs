mod commands;
// Import necessary modules
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
use colored::*; // For colored output
use std::env;

fn main() {
    // Display the logo only for general help
    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        display_logo();
    }
    println!("Current working directory: {:?}", env::current_dir().unwrap());
    // Check if the ~/.locker folder exists
    let user_dirs = UserDirs::new().expect("Unable to access the user directory");
    let locker_dir = user_dirs.home_dir().join(".locker");

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
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .num_args(1)
                .required(true)
                .help("Name of the secret"))
            .arg(Arg::new("value")
                .short('v')
                .long("value")
                .num_args(1)
                .required(false)
                .help("Value of the secret to encrypt")),
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
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .num_args(1)
                .required(true)
                .help("Name of the secret to decrypt"))
            .arg(Arg::new("clipboard")
                .short('c')
                .long("clipboard")
                .action(clap::ArgAction::SetTrue)
                .required(false)
                .help("Copies the decrypted secret to the clipboard")),
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
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .num_args(1)
                .required(true)
                .help("Name of the secret to delete")),
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        display_logo(); // Display the logo only for the init command
        if let Some(passphrase) = matches.get_one::<String>("passphrase") {
            let salt = b"smartlocker_salt"; // You can customize the salt
            let key = derive_key_from_passphrase(passphrase, salt);

            let key_path = locker_dir.join("locker.key");
            fs::write(&key_path, key).expect("Error writing the key");
            println!("{}", format!("✅ Key generated from the passphrase and saved: {:?}", key_path).green());
        } else {
            init_locker(); // Call the existing function to generate a random key
            println!("{}", "✅ Vault initialized successfully!".green());
        }
    } else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let name = matches.get_one::<String>("name").unwrap();
        let value = if let Some(value) = matches.get_one::<String>("value") {
            value.clone()
        } else {
            // Read from stdin if --value is not provided
            let mut input = String::new();
            std::io::stdin()
                .read_to_string(&mut input)
                .expect("Error reading from stdin");
            input.trim().to_string()
        };
        encrypt(&value, name);
        println!("{}", format!("✅ Secret '{}' encrypted successfully!", name).green());
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let name = matches.get_one::<String>("name").unwrap();
        let decrypted_value = decrypt(name);
        if matches.get_flag("clipboard") {
            if cfg!(target_os = "linux") && std::env::var("WSL_DISTRO_NAME").is_ok() {
                // Use clip.exe for WSL
                use std::process::{Command, Stdio};
                let mut child = Command::new("clip.exe")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Unable to execute clip.exe");
                {
                    let stdin = child.stdin.as_mut().expect("Unable to access stdin");
                    use std::io::Write;
                    stdin
                        .write_all(decrypted_value.as_bytes())
                        .expect("Error writing to clip.exe");
                }
                child.wait().expect("Error executing clip.exe");
                println!("{}", "✅ Secret copied to Windows clipboard!".green());
            } else {
                // Copy to Linux clipboard
                use copypasta::{ClipboardContext, ClipboardProvider};
                let mut ctx = ClipboardContext::new().expect("Unable to access the clipboard");
                ctx.set_contents(decrypted_value.clone())
                    .expect("Error copying to the clipboard");
                println!("{}", "✅ Secret copied to the clipboard!".green());
            }
        }
    } else if matches.subcommand_matches("list").is_some() {
        let secrets = list_secrets(&locker_dir);
        if secrets.is_empty() {
            println!("{}", "⚠️ No secrets found.".yellow());
        } else {
            println!("{}", "🔒 Available secrets:".blue());
            for secret in secrets {
                println!("{}", secret);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.get_one::<String>("name").unwrap();
        remove_secret(name);
        println!("{}", format!("✅ Secret '{}' deleted successfully!", name).green());
    }
}

fn display_logo() {
    println!("{}", "🦀🔐 SmartLocker - Secure your secrets with Rust!".bold().green());
}