use crate::expression::token::Token;

use super::token_builder::TokenBuilder;

pub struct Tokenizer {
    input: String,

    cursor: usize,
}

// имплементация трейта (интерфейса) итератора
impl Iterator for Tokenizer {
    type Item = Token;

    // на каждой итерации берем срез строки от текущего положения курсора и пытаемся найти в нем следующий токен
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.input.len() {
            return None;
        }

        let cursor = TokenBuilder::new(self.input.clone(), self.cursor);
        let token = cursor.next();

        self.cursor += token.len();

        Some(token)

        // let substring = String::from(&self.input[self.cursor..]);
        // println!("substr {}", substring);
        //
        // let token = self.get_next_token(substring);
        //
        // if let Some(token) = &token {
        //     self.cursor += token.len();
        // }
        //
        // token
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

    pub fn parse(self) -> Vec<Token> {
        self.into_iter().collect::<Vec<Token>>()
    }

    // получение следующего токена со среза строки (для итератора)
    fn get_next_token(&mut self, s: String) -> Option<Token> {
        let mut token = TokenBuilder::new(self.input.clone(), self.cursor);
        let mut iter = s.chars().into_iter();

        while token.is_filled() == false {
            let c = iter.next();
            match c {
                Some(c) => token.push(c),
                None => break,
            }
        }

        token.build()
    }
}
