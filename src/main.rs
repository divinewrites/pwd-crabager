mod cli;
mod manager;

use cli::Action;
use manager::PasswordManager;
use structopt::StructOpt;

fn main() {
    let db_name = "pwd-crabager.db";
    let mut password_manager = PasswordManager::new(db_name);

    let args = cli::CommandLineArgs::from_args();

    let action = args.action;

    match action {
        Action::Create { site, force } => {
            password_manager.prompt(&site, force.unwrap_or(false));
        }
        Action::Edit { site } => {
            password_manager.edit_password(&site);
        }
        Action::Generate {} => {
            let password = password_manager.generate_password();
            println!("Generated password: {}", password)
        }
        Action::List {} => {
            let _ = password_manager.list_sites(db_name);
        }
    }
}
