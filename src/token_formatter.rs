use crate::tokenizer;

const INDENT: &str = "  ";

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
