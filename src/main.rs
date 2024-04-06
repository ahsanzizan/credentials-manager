mod credentials_manager;

use credentials_manager::CredentialsManager;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command.");
        process::exit(1);
    }

    let command = &args[1];
    let mut credentials_manager = CredentialsManager::new();

    match command.as_str() {
        "add" => {
            if args.len() != 4 {
                println!("Usage: add <key> <password>");
                process::exit(1);
            }
            let key = &args[2];
            let password = &args[3];
            credentials_manager.add_credential(key, password);

            match credentials_manager.save_to_file(&format!("{key}.txt")) {
                Ok(_) => println!("Credentials saved to file."),
                Err(e) => println!("Error saving to file: {}", e),
            }
        }
        "get" => {
            if args.len() != 3 {
                println!("Usage: get <key>.txt");
                process::exit(1);
            }
            let key = &args[2];
            match credentials_manager.get_credential(key) {
                Some(password) => println!("Password: {}", password),
                None => println!("No password found for key: {}", key),
            }
        }
        _ => {
            println!("Unknown command: {}", command);
            process::exit(1);
        }
    }
}
