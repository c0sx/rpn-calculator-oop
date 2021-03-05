use crate::expression::token::Token;

use super::cursor::TokenCursor;

pub struct Tokenizer {
    input: String,

    cursor: usize,
}

// имплементация трейта (интерфейса) итератора
impl Iterator for Tokenizer {
    type Item = Token;

    // на каждой итерации берем срез строки от текущего положения курсора и пытаемся найти в нем следующий токен
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.input.chars().count() {
            return None;
        }

        let mut cursor = TokenCursor::new(self.input.clone(), self.cursor);
        let token = cursor.next();

        self.cursor += token.len();

        Some(token)
    }
}

impl Tokenizer {
    pub fn from(input: &String) -> Tokenizer {
        let without_whitespaces = input.split_whitespace().collect::<String>();

        Tokenizer {
            input: without_whitespaces,
            cursor: 0,
        }
    }

    // собираем токены с помощью итератора
    pub fn parse(self) -> Vec<Token> {
        self.into_iter().collect::<Vec<Token>>()
    }
}
