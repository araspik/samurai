//! # Frontend for SMake.
//!   
//! The power of SMake is in the library, but this is the glue that connects
//! that power to the CLI (and so to the users).
//!   
//! Author: ARaspiK  
//! License: MIT  

extern crate getopts;

use smake;
use std::{env, error::Error, process::exit};
use getopts as getopt;

struct Opts {
    opts: getopt::Options,
    prog: String,
    path: String,
    args: Vec<String>,
}

fn print_help(prog: &String, opts: &getopt::Options) {
    let usage = format!("Usage: {} [options] TARGET", prog);
    eprint!("{}", opts.usage(&usage));
}

fn parse_opts() -> Result<Option<Opts>, getopt::Fail> {
    // Get args
    let args = env::args().collect::<Vec<_>>();
    let prog = args[0].to_string();

    // Set up options
    let mut opts = getopt::Options::new();
    opts.optopt( "f", "file", "The SMakefile to read from", "PATH");
    opts.optflag("h", "help", "Provides help information");

    // Parse
    let matches = opts.parse(&args[1..])?;

    // Special case: help info
    if matches.opt_present("h") {
        print_help(&prog, &opts);
        return Ok(None);
    }

    // Gather args and return
    Ok(Some(Opts {
        opts,
        prog,
        path: matches.opt_str("f")
                .unwrap_or("./SMakefile".to_string()),
        args: matches.free,
    }))
}

fn work(opts: Opts) -> Result<(), Box<dyn Error>> {
    if opts.args.is_empty() {
        eprintln!("Not enough arguments provided!");
        print_help(&opts.prog, &opts.opts);
        return Ok(());
    }

    let mut file = smake::File::from_file(&opts.path)
        .map_err(|err| Box::new(err) as Box<dyn Error>)?;

    for target in opts.args.iter() {
        if let Some(rule) = file.rules.remove(target) {
            rule.map(|rule| println!("{}", rule))
                .map_err(|err| Box::new(err) as Box<dyn Error>)?
        } else {
            eprintln!("Target \"{}\" not found!", target);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = parse_opts()
        .map_err(|err| Box::new(err) as Box<dyn Error>)
        .and_then(|opts| opts.map(|opts| work(opts)).unwrap_or(Ok(()))) {
        eprintln!("{}", err);
        exit(1)
    }
}
