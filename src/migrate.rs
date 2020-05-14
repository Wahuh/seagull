use anyhow::{Context, Result};
use postgres::{Client, NoTls};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Migration {
    pub sql: String,
    pub path: PathBuf,
    pub version_number: i32,
}

impl Migration {
    pub fn from_file(path: PathBuf) -> Result<Migration> {
        let sql = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read sql for {}", &path.display()))?;

        let version_number = path
            .file_name()
            .with_context(|| format!("Failed to read filename for {}", &path.display()))?
            .to_str()
            .with_context(|| {
                format!(
                    "Failed to convert filename for {} to UTF-8",
                    &path.display()
                )
            })?
            .chars()
            .nth(1)
            .with_context(|| {
                format!("Failed to extract the version number from the migration file")
            })?
            .to_digit(10)
            .with_context(|| "The second character of the filename is an invalid version number")?;

        let migration = Migration {
            path,
            sql,
            version_number: version_number as i32,
        };
        Ok(migration)
    }
}

pub struct Runner {
    connection_string: String,
}

impl Runner {
    pub fn new(connection_string: String) -> Runner {
        Runner { connection_string }
    }

    pub fn downgrade(&self) -> Result<()> {
        let mut client = self.connect()?;
        client
            .batch_execute(
                "DROP SCHEMA public CASCADE;
            CREATE SCHEMA public;
        GRANT ALL ON SCHEMA public TO postgres;
        GRANT ALL ON SCHEMA public TO public;
        COMMENT ON SCHEMA public IS 'standard public schema';",
            )
            .context("Failed to wipe database")?;
        Ok(())
    }

    fn connect(&self) -> Result<Client> {
        let client = Client::connect(&self.connection_string, NoTls)
            .context("Failed to connect to database")?;
        Ok(client)
    }

    pub fn setup_migration_history_table(&self) -> Result<()> {
        let mut client = self.connect()?;
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS __migration_history ( version_number INTEGER )",
            )
            .with_context(|| "Failed to create __migration_history table")?;
        Ok(())
    }

    pub fn run_migrations(&self, migrations: Vec<Migration>) -> Result<()> {
        let mut client = self.connect()?;
        client.batch_execute("BEGIN")?;

        let row = client
            .query_one("SELECT version_number from __migration_history", &[])
            .ok();

        let version_number: i32 = match row {
            Some(row) => row.get("version_number"),
            None => 0,
        };

        let mut unapplied_migrations: Vec<_> = migrations
            .iter()
            .filter(|migration| migration.version_number > version_number)
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
}

pub fn find_migrations<P: AsRef<Path>>(dir_path: P) -> Result<Vec<Migration>> {
    let entries =
        fs::read_dir(dir_path).with_context(|| format!("Failed to find the directory at"))?;

    let mut migrations = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let migration = Migration::from_file(path)?;
        migrations.push(migration);
    }

    Ok(migrations)
}
