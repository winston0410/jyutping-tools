use clap::{Command, ErrorKind, Parser};
use std::collections::HashMap;
use std::path::Path;
mod lib;
use std::fs::{read_to_string, write};

/// Clean up plain text retrived from Wikipedia. This will transform the file in-place.
#[derive(Parser, Debug)]
struct Args {
    /// Path for the file for preprocessing
    path: String,
    /// Path for safe variant file. Expectating a json for 1 to 1 mapping.
    #[clap(short, long)]
    variant_file: Option<String>
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
            //Not sure if it is alright to remove all ascii characters
            let handled_ascii = lib::remove_ascii(handled_parenthesis);

            //Normalize by replacing identical variant
            let normalized = match args.variant_file {
                Some(p) => {
                    match read_to_string(&p) {
                        Ok(variant_content) => {
                            let variant_map: HashMap<String, String> = serde_json::from_str(&variant_content).unwrap();

                            //Use chars() to iterate only once, instead of iterating multiple times with .replace()
                            handled_ascii.chars().map(|char| {
                                let str = char.to_string();
                                match variant_map.get(&str) {
                                    Some(variant) => {
                                        variant.to_owned()
                                    },
                                    None => {
                                        str
                                    }
                                }
                            }).collect()
                        },
                        Err(x) => {
                            cmd.error(ErrorKind::Io, x);
                            //Use unreachable!() here?
                            handled_ascii
                        }
                    }
                },
                None => {
                    handled_ascii
                }
            };

            write(&path, 
                normalized
            ).unwrap();
        }
        Err(x) => {
            cmd.error(ErrorKind::Io, x);
        }
    }
}
