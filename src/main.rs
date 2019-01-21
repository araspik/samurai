//! # Frontend for SMake.
//!   
//! The power of SMake is in the library, but this is the glue that connects
//! that power to the CLI (and so to the users).
//!   
//! Author: ARaspiK  
//! License: MIT  

extern crate getopts;

use smake;
use std::{env, fs};
use std::path::PathBuf;
use getopts::Options;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut opts = Options::new();
    opts.optopt( "f", "file", "The SMakefile to read from", "PATH");
    opts.optflag("h", "help", "Provides help information");

    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|e| panic!(e.to_string()));

    if matches.opt_present("h") || matches.free.is_empty() {
        let usage = format!("Usage: {} [options] TARGETS", args[0]);
        print!("{}", opts.usage(&usage));
        return;
    }

    let file = fs::File::open(PathBuf::from(matches.opt_str("f")
            .unwrap_or("./SMakefile".to_string())))
        .unwrap_or_else(|e| panic!(e.to_string()));

    let rules = smake::File::from_reader(file)
        .unwrap_or_else(|e| panic!(e.to_string()));

    let rule = rules.get(matches.free.get(0).unwrap());

    if let Some(rule) = rule {
        match rule {
            Ok(rule) => println!("{}", rule),
            Err(err) => println!("Invalid rule: {}", err.to_string()),
        }
    } else {
        println!("Target {} not found!", matches.free.get(0).unwrap());
    }
}
