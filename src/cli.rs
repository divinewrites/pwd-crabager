use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Create a new password for the site provided
    Create {
        #[structopt()]
        site: String,
        #[structopt(parse(try_from_str))]
        force: Option<bool>,
    },

    /// Edit the password of a site
    Edit {
        #[structopt()]
        site: String,
    },

    /// Generates a secure password for you
    Generate,

    /// List all current sites a password was created for
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pwd-crabager",
    about = "A simple & 'secure' command line password manager written in Rust🦀"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}

