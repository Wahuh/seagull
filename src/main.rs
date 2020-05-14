mod cli;
mod config;
mod init;
mod migrate;
mod poop;

use anyhow::Result;
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
    Remigrate {},
    /// Sets up the "seagull.toml" config file
    Init {},
}

fn main() -> Result<()> {
    let args = Seagull::from_args();

    match args {
        Seagull::Poop { description } => {
            cli::handle_poop(description)?;
        }
        Seagull::Migrate {} => {
            cli::handle_migrate()?;
        }
        Seagull::Remigrate {} => {
            cli::handle_remigrate()?;
        }
        Seagull::Init {} => {
            cli::handle_init()?;
        }
    }

    Ok(())
}
