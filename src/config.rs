use crate::error::CliError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

#[derive(Deserialize, Serialize)]
pub struct Database {
    #[serde(default)]
    pub host: String,

    #[serde(default)]
    pub password: String,

    #[serde(default)]
    pub port: i32,

    #[serde(default)]
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Migrations {
    #[serde(default)]
    pub dir_path: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub migrations: Migrations,

    #[serde(default)]
    pub database: Database,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let contents = fs::read_to_string(path).with_context(|| {
            format!("Failed to find seagull.toml config file. Run `seagull init` to create it.")
        })?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn from_input() -> Result<Config, CliError> {
        print!("What's your database host? ");
        io::stdout().flush()?;
        let mut host = String::new();
        io::stdin().read_line(&mut host)?;

        print!("What's your database username? ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;

        print!("What's your database password? ");
        io::stdout().flush()?;
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;

        print!("What's your database port? ");
        io::stdout().flush()?;
        let mut port = String::new();
        io::stdin().read_line(&mut port)?;
        let port: i32 = port
            .trim()
            .parse()
            .expect("Please enter a positive port number");

        let config = Config {
            database: Database {
                host: host.trim().to_string(),
                username: username.trim().to_string(),
                port,
                password: password.trim().to_string(),
            },
            migrations: Migrations::default(),
        };
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            migrations: Migrations {
                dir_path: String::from("migrations"),
            },
            database: Database {
                host: String::from("postgres"),
                username: String::from("postgres"),
                password: String::from("postgres"),
                port: 5432,
            },
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            host: String::from("postgres"),
            username: String::from("postgres"),
            password: String::from("postgres"),
            port: 5432,
        }
    }
}

impl Default for Migrations {
    fn default() -> Self {
        Migrations {
            dir_path: String::from("migrations"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::error::CliError;
    use io::Write;
    use std::io;
    use tempfile::NamedTempFile;

    #[test]
    fn it_reads_from_toml_config_file() -> Result<(), CliError> {
        let config = b"
            [database] \n
            host = \"postgres_host\" \n
            port = 6969 \n
            username = \"Chuck\" \n
            password = \"Norris\" \n
        ";
        let mut file = NamedTempFile::new()?;
        file.write_all(config)?;

        let config = Config::from_file(file.path())?;
        assert_eq!("postgres_host", config.database.host);
        assert_eq!(6969, config.database.port);
        assert_eq!("Chuck", config.database.username);
        assert_eq!("Norris", config.database.password);
        assert_eq!("migrations", config.migrations.dir_path);
        Ok(())
    }
}
