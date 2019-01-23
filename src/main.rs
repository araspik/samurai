//! # Frontend for SMake.
//!   
//! The power of SMake is in the library, but this is the glue that connects
//! that power to the CLI (and so to the users).
//!   
//! Author: ARaspiK  
//! License: MIT  

#[macro_use] extern crate custom_error;
extern crate getopts;

use smake;
use std::{env, error::Error, fmt, process::exit};
use getopts as getopt;

custom_error!{ ErrStr<T>
    Data{data: T} = "{data}"
}

impl<T> ErrStr<T>
        where T: fmt::Display {
    pub fn new(data: T) -> Self {
        ErrStr::Data {data}
    }

    pub fn result<R>(data: T) -> Result<R, Self> {
        Err(ErrStr::Data {data})
    }
}

fn box_err<'a, T: Error + 'a>(err: T) -> Box<dyn Error + 'a> {
    Box::new(err) as Box<dyn Error + 'a>
}

struct Opts {
    path: String,
    targets: Vec<String>,
}

fn print_help(prog: &String, opts: &getopt::Options) {
    let usage = format!("Usage: {} [options] TARGET", prog);
    eprint!("{}", opts.usage(&usage));
}

fn parse_opts() -> Result<Option<Opts>, Box<dyn Error>> {
    // Get args
    let args = env::args().collect::<Vec<_>>();
    let prog = args[0].to_string();

    // Set up options
    let mut opts = getopt::Options::new();
    opts.optopt( "f", "file", "The SMakefile to read from", "PATH");
    opts.optflag("h", "help", "Provides help information");

    // Parse
    let matches = opts.parse(&args[1..]).map_err(box_err)?;

    // Special case: help info
    if matches.opt_present("h") || matches.free.is_empty() {
        print_help(&prog, &opts);
        return Ok(None);
    }

    // Gather args and return
    Ok(Some(Opts {
        path: matches.opt_str("f").unwrap_or("SMakefile".to_string()),
        targets: matches.free,
    }))
}

fn work(opts: Opts) -> Result<(), Box<dyn Error>> {
    // Parse file
    let file = smake::File::from_file(&opts.path)?;

    // For every target
    for target in opts.targets.iter() {
        let rule = file.rules.get(target)
            .map_or_else(
                || ErrStr::result(format!("Target \"{}\" not found!", target))
                    .map_err(box_err),
                |rule| Ok(rule))?;
        println!("{}", rule);
    }

    Ok(())
}

fn main() {
    if let Err(err) = parse_opts()
            .and_then(|opts| opts.map(|opts| work(opts)).unwrap_or(Ok(()))) {
        eprintln!("{}", err);
        exit(1)
    }
}
