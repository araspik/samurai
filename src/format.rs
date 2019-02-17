//! Formats define different formats to parse from.
//!
//! A format specifies which `Makefile`-like format to use when parsing a file.
//! Different formats have different features, and this allows specializing for
//! each format.
//!
//! All formats implement `Format`. This trait provides parsing routines, as
//! well as some related information.

use crate::target::Target;

use regex::Regex;

use std::error::Error;
use std::path::Path;

/// Defines specializations for a given format.
pub trait Format {
    /// The error type when parsing.
    type ParseErr: Error;

    /// Returns a regex which matches valid file names.
    /// This used when searching for a file to use.
    fn file_name() -> Regex;

    /// Parses the file at the given path, outputting into the given list.
    /// The targets are not finalized - finalization will be done later.
    ///
    /// The function will panic if the file does not exist or cannot be read
    /// from.
    fn parse<P: AsRef<Path>>(path: P, output: &mut Vec<Target>) -> Result<(), Self::ParseErr>;
}
