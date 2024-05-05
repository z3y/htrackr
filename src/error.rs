use core::fmt;


pub struct CliError(pub String);

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CliError {

    pub fn new(err: &str) -> CliError {
        CliError(err.to_owned())
    }
    
}

impl std::error::Error for CliError {}

impl From<rusqlite::Error> for CliError {
    fn from(err: rusqlite::Error) -> Self {
        CliError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for CliError {
    fn from(err: std::num::ParseIntError) -> Self {
        CliError(err.to_string())
    }
}

