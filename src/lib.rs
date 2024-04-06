use std::{
    fs,
    io::{self, Write},
};

use thiserror::Error;
use token_formatter::TokenFormatter;

pub mod nodes;
mod token_formatter;
pub mod tokenizer;

#[derive(Error, Debug)]
pub enum FormatJsonError {
    #[error("File not found")]
    FileNotFound,
    #[error("IO Error {0}")]
    IOError(#[from] io::Error),
    #[error("Input file had invalid JSON")]
    InvalidSyntax(usize, char),
    #[error("Unknown error")]
    Unknown,
}

pub fn format_json_file(filepath: &str) -> Result<(), FormatJsonError> {
    let json = fs::read_to_string(&filepath).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            return FormatJsonError::FileNotFound;
        }
        return FormatJsonError::Unknown;
    })?;

    let tokens = tokenizer::tokenize(&json)?;
    let formatter = TokenFormatter::new(tokens.into_iter());
    let mut file = io::BufWriter::new(fs::File::create(filepath)?);
    for formatted_token in formatter {
        file.write(formatted_token.as_bytes())?;
    }
    Ok(())
}

pub fn format_json(contents: &str) -> Result<String, FormatJsonError> {
    let tokens = tokenizer::tokenize(contents)?;
    let formatted_string = TokenFormatter::new(tokens.into_iter()).collect();
    return Ok(formatted_string);
}
