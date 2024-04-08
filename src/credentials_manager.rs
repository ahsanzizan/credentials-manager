use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

pub(crate) struct Credentials {
    pub username: String,
    pub password: String,
}

pub(crate) struct CredentialsManager {
    credentials: HashMap<String, Credentials>,
}

// Shifting values for trithemius cipher
const CIPHER_SHIFT: u8 = 4;

impl CredentialsManager {
    pub fn new() -> CredentialsManager {
        CredentialsManager {
            credentials: HashMap::new(),
        }
    }

    pub fn add_credential(&mut self, key: &str, username: &str, password: &str) {
        let encrypted_password = self.encrypt_password(password);
        self.credentials.insert(
            key.to_string(),
            Credentials {
                username: username.to_string(),
                password: encrypted_password,
            },
        );
    }

    pub fn encrypt_password(&self, password: &str) -> String {
        let chars: Vec<char> = password.chars().collect();

        let mut result = String::new();

        for &c in chars.iter() {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };
                // 26 is the total number of letters
                let encrypted_char = ((((c as u8 - base) + CIPHER_SHIFT) % 26) + base) as char;

                result.push(encrypted_char);
            } else {
                result.push(c);
            }
        }

        result
    }

    fn decrypt_password(&self, encrypted_password: &str) -> String {
        let chars: Vec<char> = encrypted_password.chars().collect();

        let mut result = String::new();

        for &c in chars.iter() {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() {
                    'a' as u8
                } else {
                    'A' as u8
                };
                // 26 is the total number of letters
                let decrypted_char = ((((c as u8 - base) + 26 - CIPHER_SHIFT) % 26) + base) as char;

                result.push(decrypted_char);
            } else {
                result.push(c);
            }
        }

        result
    }

    pub fn get_credential(&mut self, key: &str) -> Option<Credentials> {
        match self.load_from_file(&format!("{key}.txt")) {
            Ok(credentials_manager) => self.credentials = credentials_manager.credentials,
            Err(e) => println!("Error getting credential: {}", e),
        };
        self.credentials.get(key).map(|s| Credentials {
            username: s.username.to_string(),
            password: self.decrypt_password(&s.password),
        })
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Error> {
        let mut file = File::create(filename)?;
        for (_, value) in &self.credentials {
            writeln!(file, "{} {} {}", value.key, value.username, value.password)?;
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
            let username = parts.next().unwrap().to_string();
            let password = parts.next().unwrap().to_string();

            credentials_manager
                .credentials
                .insert(key, Credentials { username, password });
        }

        Ok(credentials_manager)
    }
}
