use crate::{
    config::Config,
    init::create_toml_config_file,
    migrate::{find_migrations, Runner},
    poop::{create_sql_file, get_next_version_number},
};
use anyhow::Result;
use std::{fs, path::Path};

pub fn handle_init() -> Result<()> {
    create_toml_config_file()?;
    Ok(())
}

pub fn handle_remigrate() -> Result<()> {
    let config = Config::from_file("seagull.toml")?;
    let connection_string = config.postgres.connection_string();
    let migrations = find_migrations(&config.migrations.dir_path)?;
    let runner = Runner::new(connection_string);
    runner.downgrade()?;
    runner.setup_migration_history_table()?;
    runner.run_migrations(migrations)?;
    Ok(())
}

pub fn handle_poop(description: String) -> Result<()> {
    let config = Config::from_file("seagull.toml")?;

    let dir_path = Path::new(&config.migrations.dir_path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
        create_sql_file(&description, 1)?;
    } else {
        let next_version_number = get_next_version_number(&config.migrations.dir_path)?;
        create_sql_file(&description, next_version_number)?;
    }
    Ok(())
}

pub fn handle_migrate() -> Result<()> {
    let config = Config::from_file("seagull.toml")?;
    let migrations = find_migrations(&config.migrations.dir_path)?;
    let connection_string = config.postgres.connection_string();
    let runner = Runner::new(connection_string);
    runner.setup_migration_history_table()?;
    runner.run_migrations(migrations)?;
    Ok(())
}
