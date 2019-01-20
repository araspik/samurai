/*! File: Representation of a SMakefile.
  * 
  * This represents SMakefiles, which currently only consist of rules.
  * 
  * Author: ARaspiK
  * License: MIT
  */

use super::rule::Rule;

use std::io;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Representation of a SMakefile.
pub struct File {
    path: PathBuf,
    rules: HashMap<String, io::Result<Rule>>,
}

impl File {
    /// Returns a reference to a rule if it exists.
    pub fn get(&self, name: &String) -> Option<&io::Result<Rule>> {
        self.rules.get(name)
    }

    /// Returns a mutable reference to a rule if it exists.
    pub fn get_mut(&mut self, name: &String) -> Option<&mut io::Result<Rule>> {
        self.rules.get_mut(name)
    }

    /// Returns the path to the SMakefile.
    pub fn path<'a>(&'a self) -> &'a Path {
        self.path.as_path()
    }
}
