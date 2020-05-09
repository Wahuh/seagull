mod cli;
mod config;
mod error;
mod init;
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

    /// Sets up the "seagull.toml" config file
    Init {},
}

fn main() -> Result<()> {
    let args = Seagull::from_args();

    match args {
        Seagull::Poop { description } => {
            cli::handle_poop(description)?;
        }
        Seagull::Migrate {} => {}
        Seagull::Init {} => {
            init::scaffold_config_file()?;
        }
    }

    Ok(())
}
