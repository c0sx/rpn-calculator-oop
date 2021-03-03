use crate::tokenizer::tokenizer::Tokenizer;

use super::token::Token;

#[derive(Debug)]
pub struct InfixExpression {
    tokens: Vec<Token>,
    cursor: usize,
}

// имплементация трейта (интерфейса) итератора
impl Iterator for InfixExpression {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
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

impl InfixExpression {
    pub fn from(tokenizer: Tokenizer) -> InfixExpression {
        InfixExpression {
            tokens: tokenizer.parse(),
            cursor: 0,
        }
    }
}
