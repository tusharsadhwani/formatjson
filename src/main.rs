use std::{env, fs, process};

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
    let formatted_json = match formatted_json {
        Ok(json) => json,
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

    // Write the formatted json to the same file.
    if fs::write(&filepath, formatted_json).is_err() {
        eprintln!(
            "Error: Failed to write formatted JSON back to the file: {}",
            filepath
        );
        process::exit(3);
    }

    eprintln!("Successfully formatted {}", filepath);
}
