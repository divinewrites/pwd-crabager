use std::{
    error::Error,
    io::{self, Write},
    path::Path,
};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rand::{thread_rng, Rng};
use rpassword::read_password;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct PasswordData {
    id: String,
    value: String,
}

pub struct PasswordManager {
    pub database: PickleDb,
}

impl PasswordManager {
    pub fn new(db_name: &str) -> Self {
        PasswordManager {
            database: Self::initialize_db(db_name),
        }
    }

    pub fn write_to_db(&mut self, data: &PasswordData) -> Result<()> {
        self.database.set(&data.id, &data.value)?;
        Ok(())
    }

    pub fn prompt(&mut self, site: &str, generated: bool) {
        if let Some(_existing_password) = self.database.get::<String>(site) {
            println!("Use the CLI argument 'Edit' to edit the existing password.");

            return;
        }

        if generated {
            let data = PasswordData {
                id: site.to_string(),
                value: self.generate_password(),
            };
            println!("Password: {}", data.value);

            if let Err(e) = self.write_to_db(&data) {
                eprintln!("Failed to save password: {}", e);
                std::process::exit(1);
            }

            println!("Successfully saved password for: {}", site);
            return;
        }

        print!("Enter a password: ");

        io::stdout().flush().unwrap();
        let password = read_password().unwrap_or_else(|e| {
            eprintln!("Failed to read password: {}", e);
            std::process::exit(1);
        });

        let data = PasswordData {
            id: site.to_string(),
            value: password,
        };

        if let Err(e) = self.write_to_db(&data) {
            eprintln!("Failed to save password: {}", e);
            std::process::exit(1);
        }

        println!("Successfully saved password for: {}", site);
    }

    pub fn generate_password(&self) -> String {
        let mut pass = String::new();
        let mut rng = thread_rng();
        for _ in 0..11 {
            pass.push(rng.gen_range('!'..'{'));
        }
        pass
    }

    pub fn list_sites(&self, db_name: &str) -> Result<()> {
        self.check_database_exists(db_name)?;

        let db = &self.database;

        if db.total_keys() <= 0 {
            eprintln!("Error: The database is empty. Try creating a password first!");
            std::process::exit(1);
        }

        println!("You have the following sites <-> passwords: ");
        for kv in db.iter() {
            println!("{}: {}", kv.get_key(), kv.get_value::<String>().unwrap());
        }

        Ok(())
    }

    pub fn edit_password(&mut self, site: &str) {
        if let Some(existing_password) = self.database.get::<String>(site) {
            println!("Current password: {}", existing_password);
            print!("Enter a new password for {}: ", site);

            io::stdout().flush().unwrap();
            let new_password = read_password().unwrap_or_else(|e| {
                eprintln!("Failed to read password: {}", e);
                std::process::exit(1);
            });

            let data = PasswordData {
                id: site.to_string(),
                value: new_password,
            };

            if let Err(e) = self.write_to_db(&data) {
                eprintln!("Failed to update password: {}", e);
                std::process::exit(1);
            }

            println!("Successfully updated password for: {}", site);
        } else {
            println!(
                "The site '{}' does not exist in the database. Use 'Create' to add a new password.",
                site
            );
        }
    }

    pub fn delete_password(&mut self, site: &str) {
        if let Some(_) = self.database.get::<String>(site) {
            if let Err(e) = self.database.rem(&site) {
                eprintln!("Failed to update password: {}", e);
                std::process::exit(1);
            }

            println!("Successfully deleted password for: {}", site);
        } else {
            println!("Site not found.");
            return;
        }
    }

    fn initialize_db(db_name: &str) -> PickleDb {
        if Path::new(db_name).exists() {
            Self::load_database(db_name).unwrap_or_else(|e| {
                eprintln!("Failed to load existing database: {}", e);
                std::process::exit(1);
            })
        } else {
            Self::new_database(db_name)
        }
    }

    fn load_database(db_name: &str) -> Result<PickleDb> {
        PickleDb::load(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
        .map_err(|_| "Failed to load database".into())
    }

    fn new_database(db_name: &str) -> PickleDb {
        PickleDb::new(
            db_name,
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json,
        )
    }

    fn check_database_exists(&self, db_name: &str) -> Result<()> {
        if !Path::new(db_name).exists() {
            return Err("Error: Database doesn't exist".into());
        }
        Ok(())
    }
}
