use crate::{config::Config, error::CliError};
use std::{fs::File, io::Write};

pub fn scaffold_config_file() -> Result<(), CliError> {
    let config = Config::from_input()?;
    let toml = toml::to_string(&config)?;
    let mut file = File::create("seagull.toml").unwrap();
    file.write_all(toml.as_bytes())?;
    Ok(())
}
