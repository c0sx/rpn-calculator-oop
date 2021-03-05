use crate::expression::token::Token;

use super::symbol::symbol::Symbol;

pub struct TokenCursor {
    input: String,
    cursor: usize,
}

impl TokenCursor {
    pub fn new(input: String, cursor: usize) -> TokenCursor {
        TokenCursor { input, cursor }
    }

    pub fn next(&mut self) -> Token {
        let substr = &self.input[self.cursor..];
        let mut token = String::new();

        for c in substr.chars().into_iter() {
            let symbol = Symbol::from(&c);
            let stop = match symbol {
                Symbol::Numeric => self.parse_numeric(c, &mut token),
                Symbol::MaybeUnaryOperator => self.parse_maybe_unary(c, &mut token),
                Symbol::BinaryOperator => self.parse_binary_operator(c, &mut token),
                Symbol::Brackets => self.parse_brackets(c, &mut token),
            };

            self.cursor += 1;

            if stop {
                break;
            }
        }

        Token::new(token)
    }

    fn parse_numeric(&self, c: char, token: &mut String) -> bool {
        token.push(c);

        false
    }

    fn parse_maybe_unary(&self, c: char, token: &mut String) -> bool {
        let vec = self.input.chars().collect::<Vec<char>>();
        let prev = if self.cursor > 0 {
            vec.get(self.cursor - 1)
        } else {
            None
        };

        if let Some(prev) = prev {
            let prev = Symbol::from(prev);
            if prev == Symbol::Numeric {
                return self.parse_binary_operator(c, token);
            }
        }

        return self.parse_unary_operator(c, token);
    }

    fn parse_binary_operator(&self, c: char, token: &mut String) -> bool {
        if token.chars().count() == 0 {
            token.push(c)
        }

        true
    }

    fn parse_unary_operator(&self, c: char, token: &mut String) -> bool {
        token.push(c);

        false
    }

    fn parse_brackets(&self, c: char, token: &mut String) -> bool {
        if token.chars().count() == 0 {
            token.push(c);
        }

        true
    }
}
