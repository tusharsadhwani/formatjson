//! Contains the token formatter struct, which consumes and formats tokens.
use crate::tokenizer;

/// Formatter uses 2 space indents.
pub const INDENT: &str = "  ";

/// Token formatter struct. The iterator yields formatted tokens.
///
/// Check [TokenFormatter::next] for formatting rules.
pub struct TokenFormatter<T> {
    pub tokens: T,
    indent_level: usize,
}

impl<'a, T> TokenFormatter<T>
where
    T: Iterator<Item = tokenizer::Token<'a>>,
{
    pub fn new(tokens: T) -> Self {
        Self {
            tokens,
            indent_level: 0,
        }
    }
}

impl<'a, T> Iterator for TokenFormatter<T>
where
    T: Iterator<Item = tokenizer::Token<'a>>,
{
    type Item = String;

    /// Formats each token, based on the following rules:
    /// - Before every closing bracket and brace, decrease indent level by 1,
    ///   and write a newline and the current indent.
    /// - Write the token itself, trimmed of whitespace.
    /// - After every colon, write a space.
    /// - After every comma, write a newline.
    /// - After every closing bracket and brace, increase indent level by 1,
    ///   and write a newline and current indent.
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.next()?;

        let mut formatted_token = String::new();

        macro_rules! print_indent {
            () => {
                for _ in 0..self.indent_level {
                    formatted_token.push_str(INDENT);
                }
            };
        }

        if let tokenizer::TokenType::RightBracket | tokenizer::TokenType::RightBrace =
            token.token_type
        {
            formatted_token.push('\n');
            self.indent_level = self.indent_level.saturating_sub(1);
            print_indent!();
        }

        formatted_token.push_str(&token.to_string());

        match token.token_type {
            tokenizer::TokenType::Comma => {
                formatted_token.push('\n');
                print_indent!();
            }
            tokenizer::TokenType::Colon => {
                formatted_token.push(' ');
            }
            _ => {}
        }

        if let tokenizer::TokenType::LeftBracket | tokenizer::TokenType::LeftBrace =
            token.token_type
        {
            formatted_token.push('\n');
            self.indent_level += 1;
            print_indent!();
        }

        return Some(formatted_token);
    }
}
