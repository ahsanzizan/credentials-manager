use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

pub(crate) struct CredentialsManager {
    credentials: HashMap<String, String>,
}

impl CredentialsManager {
    pub fn new() -> CredentialsManager {
        CredentialsManager {
            credentials: HashMap::new(),
        }
    }

    pub fn add_credential(&mut self, key: &str, password: &str) {
        let encrypted_password = self.encrypt_password(password);
        self.credentials.insert(key.to_string(), encrypted_password);
    }

    pub fn encrypt_password(&self, password: &str) -> String {
        // Simple encryption: reverse the password string
        password.chars().rev().collect::<String>()
    }

    pub fn decrypt_password(&self, encrypted_password: &str) -> String {
        // Simple decryption: reverse the string back
        encrypted_password.chars().rev().collect::<String>()
    }

    pub fn get_credential(&mut self, key: &str) -> Option<String> {
        match self.load_from_file(&format!("{key}.txt")) {
            Ok(credentials_manager) => self.credentials = credentials_manager.credentials,
            Err(e) => println!("Error getting credential: {}", e),
        };
        self.credentials.get(key).map(|s| self.decrypt_password(s))
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Error> {
        let mut file = File::create(filename)?;
        for (key, value) in &self.credentials {
            writeln!(file, "{} {}", key, value)?;
        }
        Ok(())
    }

    pub fn load_from_file(&self, filename: &str) -> Result<CredentialsManager, Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut credentials_manager = CredentialsManager::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap().to_string();
            let password = parts.next().unwrap().to_string();
            credentials_manager.credentials.insert(key, password);
        }

        Ok(credentials_manager)
    }
}
