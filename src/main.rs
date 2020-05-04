mod config;
mod error;
mod init;
mod poop;

use error::CliError;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "seagull", about = "PostgreSQL migration tool")]
pub enum Seagull {
    #[structopt(name = "poop")]
    Poop {
        description: String,
    },
    Migrate {},
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
