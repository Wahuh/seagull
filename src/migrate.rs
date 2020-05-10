use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Migration {
    pub sql: String,
    pub path: PathBuf,
}

pub fn find_migrations<P: AsRef<Path>>(dir_path: P) -> Result<Vec<Migration>> {
    let entries =
        fs::read_dir(dir_path).with_context(|| format!("Failed to find the directory at"))?;

    let mut migrations = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let sql = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read sql for {}", &path.display()))?;

        let migration = Migration { path, sql };
        migrations.push(migration);
    }

    Ok(migrations)
}
