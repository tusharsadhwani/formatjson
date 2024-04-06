use std::{env, process};

use formatjson;

const BOLD_RED: &str = "\x1b[1;32m";
const BOLD_GREEN: &str = "\x1b[1;32m";
const NORMAL: &str = "\x1b[m";

fn main() {
    let mut args = env::args();

    // ignore executable name itself, we care about first arg: filepath
    args.next();

    let filepath = args.next().unwrap_or_else(|| {
        eprintln!("Usage: formatjson path/to/filename.json");
        process::exit(2);
    });

    if let Err(error) = formatjson::format_json_file(&filepath) {
        eprintln!("{}Error:{} {}", BOLD_RED, NORMAL, error);
        process::exit(1);
    }
    eprintln!("{}Success:{} formatted {}", BOLD_GREEN, NORMAL, filepath);
}
