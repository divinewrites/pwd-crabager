use base64::{engine::general_purpose, Engine as _};
use csv;
use rpassword::read_password;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};

struct PasswordManager {
    passwords: HashMap<String, String>,
    sites: HashSet<String>,
}

impl PasswordManager {
    fn new() -> Self {
        PasswordManager {
            passwords: HashMap::new(),
            sites: HashSet::new(),
        }
    }

    fn write_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(path)?;

        writer.write_record(&["site", "password"])?;

        for (site, password) in &self.passwords {
            writer.write_record(&[site, password])?;
        }

        writer.flush()?;

        Ok(())
    }

    fn create_password(&mut self, site: &str) {
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

        let b64 = general_purpose::STANDARD.encode(password);

        self.passwords.insert(site.to_string(), b64);
        self.sites.insert(site.to_string());
        println!("Password created successfully for {}.", site);
    }

    fn retrieve_password(&self, site: &str) {
        if let Some(encoded_password) = self.passwords.get(site) {
            match general_purpose::STANDARD.decode(encoded_password) {
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

    fn list_sites(&self) {
        if self.sites.is_empty() {
            println!("No sites found.");
        } else {
            println!("List of sites:");
            for site in &self.sites {
                println!("{}", site);
            }
        }
    }

    fn edit_password(&mut self, site: &str) {
        match self.passwords.get_mut(site) {
            Some(password) => {
                print!("Enter the new password for {}: ", site);
                io::stdout().flush().unwrap();
                let new_password = read_password().expect("Failed to read password");
                *password = general_purpose::STANDARD.encode(new_password);
                println!("Password updated successfully for {}.", site);
            }
            None => println!("Password not found for {}.", site),
        }
    }
}

fn main() {
    let mut password_manager = PasswordManager::new();

    loop {
        print!("Enter a command (create/retrieve/list/edit/exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();

        match command {
            "exit" => {
                if let Err(err) = password_manager.write_to_file("passwords.csv") {
                    eprintln!("Failed to write passwords to file: {}", err);
                }

                println!("Exiting the password manager.");
                break;
            }
            "create" => {
                print!("Enter the site: ");
                io::stdout().flush().unwrap();
                let mut site_input = String::new();
                io::stdin()
                    .read_line(&mut site_input)
                    .expect("Failed to read line");
                let site = site_input.trim();

                password_manager.create_password(site);
            }
            "retrieve" => {
                password_manager.list_sites();
                print!("Enter the site for which you want to retrieve the password: ");
                io::stdout().flush().unwrap();
                let mut site_input = String::new();
                io::stdin()
                    .read_line(&mut site_input)
                    .expect("Failed to read line");
                let site = site_input.trim();
                password_manager.retrieve_password(site);
            }
            "list" => {
                password_manager.list_sites();
            }
            "edit" => {
                password_manager.list_sites();
                print!("Enter the site for which you want to edit the password: ");
                io::stdout().flush().unwrap();
                let mut site_input = String::new();
                io::stdin()
                    .read_line(&mut site_input)
                    .expect("Failed to read line");
                let site = site_input.trim();
                password_manager.edit_password(site);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
