//! # Prelude: Common items used across SMake.
//!
//! This include Error and Result.  
//!
//! Author: ARaspiK  
//! License: MIT  

use std::{io, path::PathBuf};
use serde_yaml;

custom_error! {pub Error
    NoFile {path: PathBuf}
        = @{format!("File \"{}\" not found!", path.to_str().unwrap())},
    Parsing{source: serde_yaml::Error}  = "Parsing error",
    Other  {source: io::Error}          = "I/O error"
}

/// A Result type for SMake.
pub type Result<T> = std::result::Result<T, Error>;

/*/// An error type for SMake.
#[derive(Debug)]
pub enum Error {
    NoFile(PathBuf),
    Parsing(serde_yaml::Error),
    Other(io::Error),
}

impl error::Error for Error {
    /// Returns a cause for this error, if any.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Parsing(err) => Some(err),
            Error::Other(err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for Error {
    /// Displays the error as a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoFile(path) => write!(f, "File \"{}\" not found!",
                path.to_str().unwrap()),
            Error::Parsing(err) => write!(f, "Parsing error: {}", err),
            Error::Other(err) => write!(f, "I/O error:  {}", err),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    /// Converts from a YAML parsing error.
    fn from(err: serde_yaml::Error) -> Self {
        Error::Parsing(err)
    }
}

impl From<io::Error> for Error {
    /// Converts from a I/O error.
    fn from(err: io::Error) -> Self {
        Error::Other(err)
    }
}*/
