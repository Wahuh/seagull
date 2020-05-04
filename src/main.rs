mod config;
mod error;
mod init;
mod poop;

use error::CliError;
use structopt::StructOpt;

#[derive(StructOpt)]
/// PostgreSQL migration tool
#[structopt(name = "seagull")]
pub enum Seagull {
    #[structopt(name = "poop")]
    Poop {
        description: String,
    },
    Migrate {},

    /// Sets up the "seagull.toml" config file
    Init {},
}

fn main() -> Result<(), CliError> {
    let args = Seagull::from_args();

    match args {
        Seagull::Poop { description } => {}
        Seagull::Migrate {} => {}
        Seagull::Init {} => {
            init::scaffold_config_file()?;
        }
    }

    Ok(())
}
