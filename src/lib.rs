extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use] extern crate custom_error;

pub mod rule;
pub mod file;
mod prelude;

#[cfg(test)]
mod test;

pub use crate::rule::Rule;
pub use crate::file::File;
pub use crate::prelude::{Error, Result};
