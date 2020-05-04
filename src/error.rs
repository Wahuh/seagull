use std::{error, fmt, io};

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    TomlSer(toml::ser::Error),
    TomlDer(toml::de::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::TomlSer(ref err) => write!(f, "TOML Serialize error: {}", err),
            CliError::TomlDer(ref err) => write!(f, "TOML Deserialize error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::TomlDer(ref err) => Some(err),
            CliError::TomlSer(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> CliError {
        CliError::TomlDer(err)
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(err: toml::ser::Error) -> CliError {
        CliError::TomlSer(err)
    }
}
