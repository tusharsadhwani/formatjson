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

    if let Err(err) = formatjson::format_json_file(&filepath) {
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

    eprintln!("Successfully formatted {}", filepath);
}
