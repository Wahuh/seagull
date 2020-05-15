use crate::{
    config::Config,
    init::create_toml_config_file,
    migrate::{find_migrations, Runner},
    poop::{create_sql_file, get_next_version_number},
};
use anyhow::Result;
use std::{fs, path::Path};
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

fn handle_init() -> Result<()> {
    create_toml_config_file()?;
    Ok(())
}

fn handle_remigrate(connection_string: String, dir_path: String) -> Result<()> {
    let migrations = find_migrations(dir_path)?;
    let runner = Runner::new(connection_string);
    runner.restore_database()?;
    runner.setup_migration_history_table()?;
    runner.run_migrations(migrations)?;
    Ok(())
}

fn handle_poop(description: String, dir_path: String) -> Result<()> {
    let dir_path = Path::new(&dir_path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
        create_sql_file(&description, 1)?;
    } else {
        let next_version_number = get_next_version_number(dir_path)?;
        create_sql_file(&description, next_version_number)?;
    }
    Ok(())
}

fn handle_migrate(connection_string: String, dir_path: String) -> Result<()> {
    let migrations = find_migrations(dir_path)?;
    let runner = Runner::new(connection_string);
    runner.setup_migration_history_table()?;
    runner.run_migrations(migrations)?;
    Ok(())
}
