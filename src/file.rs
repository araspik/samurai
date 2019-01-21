/*! File: Representation of a SMakefile.
  * 
  * This represents SMakefiles, which currently only consist of rules.
  * 
  * Author: ARaspiK
  * License: MIT
  */

use super::rule::{Rule, RuleData};

use std::io;
use std::collections::HashMap;
use serde_yaml;
use std::iter::FromIterator;

/// Representation of a SMakefile.
pub struct File {
    rules: HashMap<String, io::Result<Rule>>,
}

impl File {
    /// Parses from the given Reader.
    pub fn from_reader<R: io::Read>(reader: R) -> serde_yaml::Result<File> {
        let data: HashMap<String, RuleData>
            = serde_yaml::from_reader(reader)?;

        Ok(File {
            rules: HashMap::from_iter(data.iter()
                .map(|(name, rule)| (name.to_string(), Rule::from_data(rule))))
        })
    }

    /// Parses the string.
    pub fn from_str(text: &str) -> serde_yaml::Result<File> {
        let data: HashMap<String, RuleData>
            = serde_yaml::from_str(text)?;

        Ok(File {
            rules: HashMap::from_iter(data.iter()
                .map(|(name, rule)| (name.to_string(), Rule::from_data(rule))))
        })
    }

    /// Returns a reference to a rule if it exists.
    pub fn get(&self, name: &String) -> Option<&io::Result<Rule>> {
        self.rules.get(name)
    }

    /// Returns a mutable reference to a rule if it exists.
    pub fn get_mut(&mut self, name: &String) -> Option<&mut io::Result<Rule>> {
        self.rules.get_mut(name)
    }
}
