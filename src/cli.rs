use crate::{
    config::Config,
    poop::{create_sql_file, get_next_version_number},
};
use anyhow::Result;
use std::{fs, path::Path};

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
