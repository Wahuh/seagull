use crate::{
    config::Config,
    init::handle_init,
    migrate::{handle_migrate, handle_remigrate},
    poop::handle_poop,
};
use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
/// PostgreSQL migration tool
#[structopt(name = "seagull")]
pub enum Seagull {
    #[structopt(name = "poop")]
    Poop { description: String },
    Migrate {
        #[structopt(long)]
        database: Option<String>,

        #[structopt(long)]
        dir: Option<String>,
    },
    Remigrate {
        #[structopt(long)]
        database: Option<String>,

        #[structopt(long)]
        dir: Option<String>,
    },
    /// Sets up the "seagull.toml" config file
    Init {},
}

pub fn run(args: Seagull) -> Result<()> {
    match args {
        Seagull::Poop { description } => {
            let config = Config::from_file("seagull.toml")?;
            handle_poop(description, config.migrations.dir_path)?;
        }
        Seagull::Migrate { database, dir } => {
            let dir_path = dir.unwrap_or(String::from("migrations"));

            if let Some(c) = database {
                handle_migrate(c, dir_path)?;
            } else {
                let config = Config::from_file("seagull.toml")?;
                handle_migrate(config.postgres.connection_string(), dir_path)?;
            }
        }
        Seagull::Remigrate { database, dir } => {
            let dir_path = dir.unwrap_or(String::from("migrations"));

            if let Some(c) = database {
                handle_remigrate(c, dir_path)?;
            } else {
                let config = Config::from_file("seagull.toml")?;
                handle_remigrate(config.postgres.connection_string(), dir_path)?;
            }
        }
        Seagull::Init {} => {
            handle_init()?;
        }
    }

    Ok(())
}
