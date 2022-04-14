use clap::{Parser, Command, ErrorKind};
use std::path::Path;
mod lib;
use std::fs::{File, read_to_string};
use std::io::{BufReader, BufRead};

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
            //Parse with nom
            let transformed = lib::transform(&x);
            println!("{:?}", transformed);

            //Write file
        },
        Err(x) => {
            cmd.error(ErrorKind::Io, x);
        }
    }
}