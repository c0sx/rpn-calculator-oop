use super::token::Token;

use crate::tokens::infix_tokens::InfixTokens;

pub struct Tokenizer {
    input: String,

    cursor: usize,
}

// имплементация трейта (интерфейса) итератора
impl Iterator for Tokenizer {
    type Item = String; // String to Token?

    // на каждой итерации берем срез строки от текущего положения курсора и пытаемся найти в нем следующий токен
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.input.len() {
            return None;
        }

        let substring = String::from(&self.input[self.cursor..]);
        let token = self.get_next_token(substring);

        if let Some(token) = &token {
            self.cursor += token.len();
        }

        token
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

    pub fn parse(self) -> InfixTokens {
        let tokens = self.into_iter().collect::<Vec<String>>();

        InfixTokens::new(tokens)
    }

    // получить следующий токен
    // трансформировать унарные токены
    fn get_next_token(&mut self, s: String) -> Option<String> {
        let token = self.parse_next_token(s);
        if token.len() == 0 {
            return None;
        }

        Some(token)
    }

    // получение следующего токена со среза строки (для итератора)
    fn parse_next_token(&self, s: String) -> String {
        let mut token = Token::new();
        let mut iter = s.chars().into_iter();

        while token.is_filled() == false {
            let c = iter.next();
            match c {
                Some(c) => token.push(c),
                None => break
            }
        }

        token.to_string()
    }
}
