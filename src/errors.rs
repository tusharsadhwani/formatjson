use std::io;

use thiserror::Error;

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
    pub fn new(filepath: String, src: &str, bad_bit: miette::SourceSpan) -> Self {
        Self {
            src: miette::NamedSource::new(filepath, src.to_string()),
            bad_bit,
        }
    }
}
