use std::fmt::Display;

use crate::FormatJsonError;

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
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn tokenize(&mut self) -> Result<Vec<Token<'a>>, FormatJsonError> {
        let mut chars = self.source.char_indices();
        let mut tokens = vec![];

        while let Some((byte_offset, char)) = chars.next() {
            // special cases first: strings and numbers
            if char == '"' {
                let string_token = self
                    .extract_string(byte_offset)
                    .ok_or(FormatJsonError::InvalidSyntax(byte_offset, char))?;
                tokens.push(Token {
                    token_type: TokenType::String(string_token),
                    byte_offset,
                });
                // consume the extra tokens, till the end of the string.
                // we don't need a length check, because a string will always be
                // at least 2 in len().
                chars.nth(string_token.len() - 2);
                continue;
            } else if let '1'..='9' | '-' = char {
                let number_token = self
                    .extract_number(byte_offset)
                    .ok_or(FormatJsonError::InvalidSyntax(byte_offset, char))?;
                tokens.push(Token {
                    token_type: TokenType::Number(number_token),
                    byte_offset,
                });
                if number_token.len() >= 2 {
                    // consume the extra tokens, till the end of the number
                    chars.nth(number_token.len() - 2);
                }
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
                return Err(FormatJsonError::InvalidSyntax(byte_offset, char));
            }
        }
        Ok(tokens)
    }

    fn extract_string(&mut self, index: usize) -> Option<&'a str> {
        let slice = self.source.get(index + 1..)?;
        slice
            .find('"')
            .map(|i| i + index + 1)
            .map(|end| &self.source[index..=end])
    }
    fn extract_number(&mut self, index: usize) -> Option<&'a str> {
        let slice = self.source.get(index + 1..)?;
        slice
            .find(|char| {
                if let '1'..='9' = char {
                    return false;
                } else {
                    return true;
                }
            })
            .map(|i| i + index + 1)
            .map(|end| &self.source[index..end])
    }
}

pub fn tokenize<'a>(source: &'a str) -> Result<Vec<Token<'a>>, FormatJsonError> {
    Tokenizer::new(source).tokenize()
}
