use crate::tokenizer::tokenizer::Tokenizer;

use super::token::Token;

#[derive(Debug)]
pub struct Expression {
    tokens: Vec<Token>,
    cursor: usize,
}

// имплементация трейта (интерфейса) итератора
impl Iterator for Expression {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.tokens.len() {
            return None;
        }

        let current = self.tokens.get(self.cursor);
        if let Some(token) = current {
            self.cursor += 1;
            return Some(Token::new(token.value.clone()));
        }

        return None;
    }
}

impl Expression {
    pub fn from(tokenizer: Tokenizer) -> Expression {
        Expression {
            tokens: tokenizer.parse(),
            cursor: 0,
        }
    }
}
