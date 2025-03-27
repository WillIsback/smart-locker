// This file is the entry point of the SmartLocker application.
// It initializes the command-line interface (CLI) and handles user input.

use clap::{App, Arg, SubCommand};
use std::process;

mod commands;

fn main() {
    let matches = App::new("SmartLocker")
        .version("0.1.0")
        .author("William")
        .about("A CLI tool for encrypting and managing secrets")
        .subcommand(
            SubCommand::with_name("encrypt")
                .about("Encrypt a secret")
                .arg(Arg::with_name("secret")
                    .help("The secret to encrypt")
                    .required(true)),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .about("Decrypt a secret")
                .arg(Arg::with_name("file")
                    .help("The .slock file to decrypt")
                    .required(true)),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List all stored secrets"),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the locker and generate a key"),
        )
        .get_matches();

    match matches.subcommand() {
        ("encrypt", Some(sub_m)) => {
            let secret = sub_m.value_of("secret").unwrap();
            commands::encrypt::encrypt_secret(secret);
        }
        ("decrypt", Some(sub_m)) => {
            let file = sub_m.value_of("file").unwrap();
            commands::decrypt::decrypt_secret(file);
        }
        ("list", Some(_)) => {
            commands::list::list_secrets();
        }
        ("init", Some(_)) => {
            commands::init::initialize_locker();
        }
        _ => {
            eprintln!("Invalid command. Use --help for more information.");
            process::exit(1);
        }
    }
}