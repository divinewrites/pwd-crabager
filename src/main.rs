mod manager;
use std::io::{self, Write};

use manager::PasswordManager;

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
            "exit" => handle_exit(&mut password_manager),
            "create" => handle_create(&mut password_manager),
            "retrieve" => handle_retrieve(&password_manager),
            "list" => handle_list(&password_manager),
            "edit" => handle_edit(&mut password_manager),
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

fn handle_exit(password_manager: &mut PasswordManager) {
    if let Err(err) = password_manager.write_to_file("passwords.csv") {
        eprintln!("Failed to write passwords to file: {}", err);
    }
    println!("Exiting the password manager.");
    std::process::exit(0);
}

fn handle_create(password_manager: &mut PasswordManager) {
    print!("Enter the site: ");
    io::stdout().flush().unwrap();
    let mut site_input = String::new();
    io::stdin()
        .read_line(&mut site_input)
        .expect("Failed to read line");
    let site = site_input.trim();
    password_manager.create_password(site);
}

fn handle_retrieve(password_manager: &PasswordManager) {
    if password_manager.sites.is_empty() {
        println!("No sites found. Cannot retrieve password.");
        return;
    }

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


fn handle_list(password_manager: &PasswordManager) {
    password_manager.list_sites();
}

fn handle_edit(password_manager: &mut PasswordManager) {
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
