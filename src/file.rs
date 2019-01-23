/*! File: Representation of a SMakefile.
  * 
  * This represents SMakefiles, which currently only consist of rules.
  * 
  * Author: ARaspiK
  * License: MIT
  */

use crate as smake;
use crate::rule::{Rule, RuleData};
use std::{io, fs};
use std::path::PathBuf;
use std::collections::HashMap;
use serde_yaml;

/// Representation of a SMakefile.
pub struct File {
    pub rules: HashMap<String, smake::Result<Rule>>,
}

impl File {
    /// Parses from the given file.
    pub fn from_file(path: &String) -> smake::Result<File> {
        let path = PathBuf::from(path);
        let file = fs::File::open(&path)
            .map_err(|e| match e.kind() {
                io::ErrorKind::NotFound
                    => smake::Error::NoFile(path),
                _ => smake::Error::Other(e),
            })?;
        Self::from_reader(file)
    }

    /// Parses from the given Reader.
    pub fn from_reader<R: io::Read>(read: R) -> smake::Result<File> {
        Ok(File {
            rules: serde_yaml::from_reader::<_,HashMap<String, RuleData>>(read)?
                .iter()
                .map(|(name, rule)| (name.to_string(), Rule::from_data(rule)))
                .collect()
        })
    }

    /// Parses from the given string.
    pub fn from_str(text: &str) -> smake::Result<File> {
        Ok(File {
            rules: serde_yaml::from_str::<HashMap<String, RuleData>>(text)?
                .iter()
                .map(|(name, rule)| (name.to_string(), Rule::from_data(rule)))
                .collect()
        })
    }

    /// Returns a reference to a rule if it exists.
    pub fn get(&self, name: &String) -> Option<&smake::Result<Rule>> {
        self.rules.get(name)
    }

    /// Returns a mutable reference to a rule if it exists.
    pub fn get_mut(&mut self, name: &String)
            -> Option<&mut smake::Result<Rule>> {
        self.rules.get_mut(name)
    }
}
