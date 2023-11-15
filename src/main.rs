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
        Action::Create { site, generated } => {
            if generated.unwrap_or(String::new()) == "generated" {
                password_manager.prompt(&site, true);
                return;
            }

            password_manager.prompt(&site, false);
        }
        Action::Edit { site } => {
            password_manager.edit_password(&site);
        }
        Action::List {} => {
            let _ = password_manager.list_sites(db_name);
        }
        Action::Delete { site } => {
            password_manager.delete_password(&site);
        }
    }
}
