use std::{
    fs,
    io::{self, Write},
};

pub use errors::FormatJsonError;
use token_formatter::TokenFormatter;

pub mod errors;
pub mod nodes;
mod token_formatter;
pub mod tokenizer;

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
