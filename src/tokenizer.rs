//! Tokenizes a given JSON string, without validating its syntax.
use std::fmt::Display;

use crate::errors;

/// The kinds of tokens produced by the tokenizer.
#[derive(Debug)]
pub enum TokenType<'a> {
    String(&'a str),
    Number(&'a str),
    Boolean(bool),
    Null,
    LeftBracket,
    LeftBrace,
    RightBracket,
    RightBrace,
    Comma,
    Colon,
}

/// Tokens produced by the tokenizer.
#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub byte_offset: usize,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::String(string) => f.write_str(string),
            TokenType::Number(number) => f.write_str(number),
            TokenType::Boolean(bool) => f.write_fmt(format_args!("{}", bool)),
            TokenType::Null => f.write_str("null"),
            TokenType::LeftBracket => f.write_str("["),
            TokenType::LeftBrace => f.write_str("{"),
            TokenType::RightBracket => f.write_str("]"),
            TokenType::RightBrace => f.write_str("}"),
            TokenType::Comma => f.write_str(","),
            TokenType::Colon => f.write_str(":"),
        }
    }
}

#[derive(Debug)]
struct Tokenizer<'a> {
    pub source: &'a str,
    pub filepath: String,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str, filepath: String) -> Self {
        Self { source, filepath }
    }

    fn tokenize(&self) -> Result<Vec<Token<'a>>, errors::FormatJsonError> {
        let mut chars = self.source.char_indices();
        let mut tokens = vec![];

        while let Some((byte_offset, char)) = chars.next() {
            // special cases first: strings and numbers
            if char == '"' {
                let string_token = self.extract_string(byte_offset).ok_or(
                    errors::InvalidSyntaxDiagnostic::new(
                        self.filepath.to_string(),
                        self.source,
                        byte_offset.into(),
                    ),
                )?;
                tokens.push(Token {
                    token_type: TokenType::String(string_token),
                    byte_offset,
                });
                // consume the extra tokens, till the end of the string.
                // we don't need a length check, because a string will always be
                // at least 2 in len().
                chars.nth(string_token.chars().count() - 2);
                continue;
            } else if let '1'..='9' | '-' = char {
                let number_token = self.extract_number(byte_offset).ok_or(
                    errors::InvalidSyntaxDiagnostic::new(
                        self.filepath.to_string(),
                        self.source,
                        byte_offset.into(),
                    ),
                )?;
                tokens.push(Token {
                    token_type: TokenType::Number(number_token),
                    byte_offset,
                });
                if number_token.len() >= 2 {
                    // consume the extra tokens, till the end of the number
                    chars.nth(number_token.len() - 2);
                }
                continue;
            } else if "tfn".contains(char) {
                let special_token = self.extract_boolean_or_null(byte_offset).ok_or(
                    errors::InvalidSyntaxDiagnostic::new(
                        self.filepath.to_string(),
                        self.source,
                        byte_offset.into(),
                    ),
                )?;
                tokens.push(Token {
                    token_type: TokenType::Number(special_token),
                    byte_offset,
                });
                // consume the extra tokens, till the end of the number
                chars.nth(special_token.chars().count() - 2);
                continue;
            }

            if " \n\t".contains(char) {
                // ignore all whitespace
            } else if char == ',' {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    byte_offset,
                })
            } else if char == ':' {
                tokens.push(Token {
                    token_type: TokenType::Colon,
                    byte_offset,
                })
            } else if char == '[' {
                tokens.push(Token {
                    token_type: TokenType::LeftBracket,
                    byte_offset,
                })
            } else if char == ']' {
                tokens.push(Token {
                    token_type: TokenType::RightBracket,
                    byte_offset,
                })
            } else if char == '{' {
                tokens.push(Token {
                    token_type: TokenType::LeftBrace,
                    byte_offset,
                })
            } else if char == '}' {
                tokens.push(Token {
                    token_type: TokenType::RightBrace,
                    byte_offset,
                })
            } else {
                return Err(errors::FormatJsonError::InvalidSyntax(
                    errors::InvalidSyntaxDiagnostic::new(
                        self.filepath.to_string(),
                        self.source,
                        byte_offset.into(),
                    ),
                ));
            }
        }
        Ok(tokens)
    }

    fn extract_string(&self, index: usize) -> Option<&'a str> {
        let slice = self.source.get(index + 1..)?;
        let mut chars = slice.char_indices();
        while let Some((i, char)) = chars.next() {
            if char == '\\' {
                // skip the next character
                chars.next();
            } else if char == '"' {
                // found the closing quote. Return string.
                let end_quote_index = index + 1 + i;
                return Some(&self.source[index..=end_quote_index]);
            }
        }

        return None;
    }
    fn extract_number(&self, index: usize) -> Option<&'a str> {
        let slice = self.source.get(index + 1..)?;
        slice
            .find(|char| {
                if let '1'..='9' | '-' | '.' | 'e' | 'E' = char {
                    return false;
                }
                // unknown character: found the end of the number
                return true;
            })
            .map(|i| index + 1 + i)
            // end_index is the character that's not part of the number.
            // so the slice will not include it.
            .map(|end_index| &self.source[index..end_index])
    }
    fn extract_boolean_or_null(&self, index: usize) -> Option<&'a str> {
        let slice = self.source.get(index..)?;
        if slice.get(..4)? == "true" {
            return Some("true");
        }
        if slice.get(..5)? == "false" {
            return Some("false");
        }
        if slice.get(..4)? == "null" {
            return Some("null");
        }

        return None;
    }
}

/// Returns tokens corresponding to the source.
pub fn tokenize<'a>(
    source: &'a str,
    filepath: String,
) -> Result<Vec<Token<'a>>, errors::FormatJsonError> {
    Tokenizer::new(source, filepath).tokenize()
}
