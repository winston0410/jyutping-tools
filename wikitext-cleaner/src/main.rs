use clap::{Command, ErrorKind, Parser};
use std::path::Path;
mod lib;
use std::fs::{read_to_string, write};


/// Clean up plain text retrived from Wikipedia. This will transform the file in-place.
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
            let transformed = lib::transform(&x)
            .replace("__NOEDITSECTION__", "");
            let handled_footstop = lib::linebreak_based_on_footstop(transformed);
            let handled_excessive_linebreak = lib::remove_excessive_linebreak(handled_footstop);
            //Handle parenthesis in the very end, as linebreak would affect regex's result
            let handled_parenthesis = lib::remove_parenthesis(handled_excessive_linebreak);

            write(&path, 
                handled_parenthesis
            ).unwrap();
        }
        Err(x) => {
            cmd.error(ErrorKind::Io, x);
        }
    }
}
