use std::{fs, io};

use thiserror::Error;

pub mod nodes;
pub mod tokenizer;

#[derive(Error, Debug)]
pub enum FormatJsonError {
    #[error("File not found")]
    FileNotFound,
    #[error("Input file had invalid JSON")]
    InvalidSyntax(usize, char),
    #[error("Unknown error")]
    Unknown,
}

pub fn format_json_file(filepath: &str) -> Result<String, FormatJsonError> {
    fs::read_to_string(&filepath)
        .map(|json| format_json(&json))
        .map_err(|err| {
            if err.kind() == io::ErrorKind::NotFound {
                return FormatJsonError::FileNotFound;
            }
            return FormatJsonError::Unknown;
        })?
}

pub fn format_json(contents: &str) -> Result<String, FormatJsonError> {
    let tokens = tokenizer::tokenize(contents)?;

    let formatted_string = format_tokens(tokens);
    return Ok(formatted_string);
}

fn format_tokens(tokens: Vec<tokenizer::Token>) -> String {
    let mut output_string = String::new();
    let mut indent_level: usize = 0;

    let indent = "  ";

    macro_rules! print_indent {
        () => {
            for _ in 0..indent_level {
                output_string.push_str(indent);
            }
        };
    }

    for token in tokens {
        if let tokenizer::TokenType::RightBracket | tokenizer::TokenType::RightBrace =
            token.token_type
        {
            output_string.push('\n');
            indent_level = indent_level.saturating_sub(1);
            print_indent!();
        }

        output_string.push_str(&token.to_string());

        match token.token_type {
            tokenizer::TokenType::Comma => {
                output_string.push('\n');
                print_indent!();
            }
            tokenizer::TokenType::Colon => {
                output_string.push(' ');
            }
            _ => {}
        }

        if let tokenizer::TokenType::LeftBracket | tokenizer::TokenType::LeftBrace =
            token.token_type
        {
            output_string.push('\n');
            indent_level += 1;
            print_indent!();
        }
    }

    return output_string;
}
