use clap::{Command, ErrorKind, Parser};
use std::path::Path;
mod lib;
use std::fs::{read_to_string, write, File};
use std::io::{BufRead, BufReader};

/// Clean up plain text retrived from Wikipedia
#[derive(Parser, Debug)]
struct Args {
    /// Path for the file for preprocessing
    path: String,
}

fn main() {
    let mut cmd = Command::new("wikitext-cleaner");

    let args = Args::parse();
    let path = Path::new(&args.path);

    //TODO use buffer. Deal with this later
    let content = read_to_string(&path);

    match content {
        Ok(x) => {
            let transformed = lib::transform(&x);
            write(&path, transformed).unwrap();
        }
        Err(x) => {
            cmd.error(ErrorKind::Io, x);
        }
    }
}
