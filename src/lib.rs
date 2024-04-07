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
    #[error("File {0} not found")]
    FileNotFound(String),
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("{0}")]
    InvalidSyntax(#[from] InvalidSyntax),
    #[error("{0}")]
    Unknown(String),
}

#[derive(Error, Debug, miette::Diagnostic)]
#[error("Invalid JSON syntax")]
pub struct InvalidSyntax {
    #[source_code]
    src: miette::NamedSource<String>,
    #[label("Invalid JSON")]
    bad_bit: miette::SourceSpan,
}

impl InvalidSyntax {
    fn new(filepath: String, src: &str, bad_bit: miette::SourceSpan) -> Self {
        Self {
            src: miette::NamedSource::new(filepath, src.to_string()),
            bad_bit,
        }
    }
}

pub fn format_json_file(filepath: &str) -> Result<(), FormatJsonError> {
    let json = fs::read_to_string(&filepath).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            return FormatJsonError::FileNotFound(filepath.to_string());
        }
        return FormatJsonError::Unknown(err.to_string());
    })?;

    let tokens = tokenizer::tokenize(&json, filepath.to_string())?;
    let formatter = TokenFormatter::new(tokens.into_iter());
    let mut file = io::BufWriter::new(fs::File::create(filepath)?);
    for formatted_token in formatter {
        file.write(formatted_token.as_bytes())?;
    }
    file.write(b"\n")?;
    Ok(())
}

pub fn format_json(contents: &str, filepath: &str) -> Result<String, FormatJsonError> {
    let tokens = tokenizer::tokenize(contents, filepath.to_string())?;
    let mut formatted_string: String = TokenFormatter::new(tokens.into_iter()).collect();
    formatted_string.push('\n');
    return Ok(formatted_string);
}
