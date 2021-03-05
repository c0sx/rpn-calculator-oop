use crate::calculator::token::Token;
use crate::calculator::tokenizer::cursor::TokenCursor;

pub struct Tokenizer {
    cursor: usize,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            cursor: 0,
        }
    }

    // собираем токены с помощью итератора
    pub fn parse(&mut self, input: &String) -> Vec<Token> {
        let without_whitespaces = input.split_whitespace().collect::<String>();

        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = self.get_next_token(&without_whitespaces) {
            tokens.push(token)
        }

        tokens
    }

    fn get_next_token(&mut self, input: &String) -> Option<Token> {
        if self.cursor >= input.chars().count() {
            return None;
        }

        let mut cursor = TokenCursor::new(input.clone(), self.cursor);
        let token = cursor.next();

        self.cursor += token.len();

        Some(token)
    }
}
