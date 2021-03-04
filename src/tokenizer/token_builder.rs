use crate::expression::token::Token;

use super::numeric::Numeric;
use super::separator::Separator;

pub struct TokenBuilder {
    input: String,
    cursor: usize,

    numeric: Numeric,
    separators: Separator,
}

impl TokenBuilder {
    pub fn new(input: String, cursor: usize) -> TokenBuilder {
        TokenBuilder {
            input,
            cursor,

            numeric: Numeric::new(),
            separators: Separator::new(),
        }
    }

    pub fn next(&self) -> Token {
        let substr = &self.input[self.cursor..];
    }

    pub fn push(&mut self, c: char) {
        println!("char {} prev_token {:?}", c, self.prev_token);

        if self.numeric.is_valid(&c) {
            self.value.push(c);
            return;
        }

        if self.separators.is_maybe_numeric(&c) {
            if let Some(prev) = &self.prev_token {
                if prev.is_numeric() == false {
                    self.value.push(c);
                    return;
                }
            } else {
                self.value.push(c);
                return;
            }
        }

        if self.separators.is_valid(&c) {
            if self.value.len() == 0 {
                self.value.push(c);
            }

            self.is_filled = true;
            return;
        }

        panic!("Неправильный токен {}", c)
    }

    pub fn is_filled(&self) -> bool {
        self.is_filled
    }

    pub fn build(&self) -> Option<Token> {
        println!("build {}", self.value.clone());

        return if self.value.len() > 0 {
            Some(Token::new(self.value.clone()))
        } else {
            None
        };
    }
}
