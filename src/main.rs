use std::{env, process};

use formatjson::{self, FormatJsonError};

fn main() {
    let mut args = env::args();

    // ignore executable name itself, we care about first arg: filepath
    args.next();

    let filepath = args.next().unwrap_or_else(|| {
        eprintln!("Usage: formatjson path/to/filename.json");
        process::exit(2);
    });

    let formatted_json = formatjson::format_json_file(&filepath);
    match formatted_json {
        Ok(json) => {
            eprintln!("{}", json);
        }
        Err(err) => {
            match err {
                FormatJsonError::FileNotFound => {
                    eprintln!("Error: File {} not found", filepath)
                }
                FormatJsonError::InvalidSyntax(byte_offset, char) => {
                    eprintln!("Error: Invalid syntax on byte {} ({:?})", byte_offset, char)
                }
                _ => {
                    eprintln!("Error: {}", err.to_string())
                }
            }
            process::exit(1);
        }
    };
}
