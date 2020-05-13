use crate::{
    config::Config,
    init::create_toml_config_file,
    migrate::find_migrations,
    poop::{create_sql_file, get_next_version_number},
};
use anyhow::{Context, Result};
use postgres::{Client, NoTls};
use std::{fs, path::Path};

pub fn handle_init() -> Result<()> {
    create_toml_config_file()?;
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
    let mut client = Client::connect(&connection_string, NoTls)?;

    client
        .batch_execute("CREATE TABLE IF NOT EXISTS __migration_history ( version_number INTEGER )")
        .with_context(|| "Failed to create __migration_history table")?;

    let row = client
        .query_one("SELECT version_number from __migration_history", &[])
        .ok();

    let current_version_number: i32 = match row {
        Some(row) => row.get("version_number"),
        None => 0,
    };

    client.batch_execute("BEGIN")?;

    let mut unapplied_migrations: Vec<_> = migrations
        .iter()
        .filter(|migration| migration.version_number > current_version_number)
        .collect();

    // make sure the migrations are in ascending order by version number
    unapplied_migrations.sort_by_key(|m| m.version_number);

    // get existing migration name
    for migration in unapplied_migrations {
        println!("{}", &migration.sql);

        client.batch_execute(&migration.sql).with_context(|| {
            format!("Failed to run migration for {}", &migration.path.display())
        })?;

        client
            .execute(
                "INSERT INTO __migration_history (version_number) VALUES ($1)",
                &[&migration.version_number],
            )
            .with_context(|| {
                format!(
                    "Failed to update __migration_history table for {}",
                    &migration.path.display(),
                )
            })?;
    }

    client.batch_execute("COMMIT")?;
    Ok(())
}
