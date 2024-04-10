use std::{env, io::Read, process};

use miette::Result;

const BOLD_RED: &str = "\x1b[1;31m";
const BOLD_GREEN: &str = "\x1b[1;32m";
const NORMAL: &str = "\x1b[m";

/// Reads a filepath argument, and validates and formats the JSON file in-place.
fn main() -> Result<()> {
    let mut args = env::args();

    // ignore executable name itself, we care about first arg: filepath
    args.next();

    // Expect filepath as an arg, and print success message on finish
    if let Some(filepath) = args.next() {
        if let Err(error) = formatjson::format_json_file(&filepath) {
            handle_error(error)?;
        }
        eprintln!("{}Success:{} formatted {}", BOLD_GREEN, NORMAL, filepath);
    // Otherwise expect JSON provided through stdin, and print output on stdout
    } else {
        if let Err(error) = format_stdin() {
            handle_error(error)?;
        }
    }
    Ok(())
}

fn format_stdin() -> Result<(), formatjson::FormatJsonError> {
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer)?;
    let formatted_json = formatjson::format_json(&buffer)?;
    println!("{}", formatted_json);
    Ok(())
}

fn handle_error(error: formatjson::FormatJsonError) -> Result<()> {
    if let formatjson::FormatJsonError::InvalidSyntax(err) = error {
        return Err(err.into());
    } else {
        eprintln!("{}Error:{} {}", BOLD_RED, NORMAL, error);
        process::exit(1);
    }
}
