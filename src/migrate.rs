use anyhow::{Context, Result};
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
