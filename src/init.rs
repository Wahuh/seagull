use crate::config::Config;
use anyhow::Result;
use std::{fs::File, io::Write};

pub fn create_toml_config_file() -> Result<()> {
    let config = Config::from_input()?;
    let toml = toml::to_string(&config)?;
    let mut file = File::create("seagull.toml").unwrap();
    file.write_all(toml.as_bytes())?;
    Ok(())
}
