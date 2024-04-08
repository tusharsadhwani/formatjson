//! Validate and format JSON files.
//!
//! The `formatjson` command is available as a binary, which reads and formats
//! JSON files in-place, and library functions are available to use or extend
//! the functionality.

use std::{
    fs,
    io::{self, Write},
};

pub use errors::FormatJsonError;
use token_formatter::TokenFormatter;

pub mod errors;
pub mod token_formatter;
pub mod tokenizer;
pub mod validator;

/// Reads, formats, and overwrites the given JSON file.
///
/// Throws a [FormatJsonError] on invalid syntax, or failing to read/write the file.
pub fn format_json_file(filepath: &str) -> Result<(), FormatJsonError> {
    let source = fs::read_to_string(&filepath).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            return FormatJsonError::FileNotFound(filepath.to_string());
        }
        return FormatJsonError::Unknown(err.to_string());
    })?;

    let tokens = tokenizer::tokenize(&source, filepath.to_string())?;

    // validate the tokens before formatting them.
    if let Err(error) = validator::validate(&tokens) {
        return Err(FormatJsonError::InvalidSyntax(
            errors::InvalidSyntaxDiagnostic::new(
                filepath,
                &source,
                error.byte_offset().into(),
                error.to_string(),
            ),
        ));
    };

    let formatter = TokenFormatter::new(tokens.into_iter());
    let mut file = io::BufWriter::new(fs::File::create(filepath)?);
    for formatted_token in formatter {
        file.write(formatted_token.as_bytes())?;
    }
    file.write(b"\n")?;
    Ok(())
}

/// Returns a new, formatted JSON string.
///
/// Throws a [FormatJsonError] on invalid syntax.
pub fn format_json(contents: &str, filepath: &str) -> Result<String, FormatJsonError> {
    let tokens = tokenizer::tokenize(contents, filepath.to_string())?;
    let mut formatted_string: String = TokenFormatter::new(tokens.into_iter()).collect();
    formatted_string.push('\n');
    return Ok(formatted_string);
}
