use csv;
use rpassword::read_password;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};
use crate::base64::Base64;

pub struct PasswordManager {
    pub passwords: HashMap<String, String>,
    pub sites: HashSet<String>,
}

impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager {
            passwords: HashMap::new(),
            sites: HashSet::new(),
        }
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(path)?;

        writer.write_record(&["site", "password"])?;

        for (site, password) in &self.passwords {
            writer.write_record(&[site, password])?;
        }

        writer.flush()?;

        Ok(())
    }

    pub fn create_password(&mut self, site: &str) {
        print!("Enter the password for {}: ", site);
        io::stdout().flush().unwrap();

        let password = read_password().expect("Failed to read password");

        if !password.chars().any(char::is_uppercase)
            || !password.chars().any(char::is_lowercase)
            || !password.chars().any(char::is_numeric)
            || password.len() < 8
        {
            println!("Password must be at least 8 characters long and include uppercase, lowercase, and numeric characters.");
            return;
        }

        let encoded_password = Base64::encode(&password);

        self.passwords.insert(site.to_string(), encoded_password);
        self.sites.insert(site.to_string());
        println!("Password created successfully for {}.", site);
    }

    pub fn retrieve_password(&self, site: &str) {
        if let Some(encoded_password) = self.passwords.get(site) {
            match Base64::decode(encoded_password) {
                Ok(decoded_password) => {
                    if let Ok(password_str) = String::from_utf8(decoded_password) {
                        println!("Password for {}: {}", site, password_str);
                    } else {
                        println!("Failed to convert password to UTF-8 for site: {}", site);
                    }
                }
                Err(err) => {
                    println!("Failed to decode password for site {}: {}", site, err);
                }
            }
        } else {
            println!("Password not found for {}.", site);
        }
    }

    pub fn list_sites(&self) {
        if self.sites.is_empty() {
            println!("No sites found.");
            return;
        } else {
            println!("List of sites:");
            for site in &self.sites {
                println!("{}", site);
            }
        }
    }

    pub fn edit_password(&mut self, site: &str) {
        match self.passwords.get_mut(site) {
            Some(password) => {
                print!("Enter the new password for {}: ", site);
                io::stdout().flush().unwrap();
                let new_password = read_password().expect("Failed to read password");
                *password = Base64::encode(&new_password);
                println!("Password updated successfully for {}.", site);
            }
            None => println!("Password not found for {}.", site),
        }
    }
}
