//! Errors raised by the library.
use std::io;

use thiserror::Error;

/// Error class raised by the library.
#[derive(Error, Debug)]
pub enum FormatJsonError {
    /// Raised when you try to format a file that doesn't exist.
    #[error("File {0} not found")]
    FileNotFound(String),
    /// Other, unexpected i/o errors, such as `PermissionDenied`.
    #[error("{0}")]
    IOError(#[from] io::Error),
    /// The JSON file has invalid syntax.
    #[error("{0}")]
    InvalidSyntax(#[from] InvalidSyntaxDiagnostic),
    /// Unexpected error.
    #[error("{0}")]
    Unknown(String),
}

/// Creates a [miette::Diagnostic] pointing at an invalid JSON syntax.
#[derive(Error, Debug, miette::Diagnostic)]
#[error("{error_message}")]
pub struct InvalidSyntaxDiagnostic {
    #[source_code]
    src: miette::NamedSource<String>,
    #[label("Error here")]
    bad_bit: miette::SourceSpan,
    error_message: String,
}

impl InvalidSyntaxDiagnostic {
    pub fn new(
        filepath: &str,
        src: &str,
        bad_bit: miette::SourceSpan,
        error_message: String,
    ) -> Self {
        Self {
            src: miette::NamedSource::new(filepath, src.to_string()),
            bad_bit,
            error_message,
        }
    }
}
