use crate::expression::token::Token;

use super::translator::Translator;

pub struct Numeric {
    output_queue: Vec<String>,
    stack: Vec<String>,
}

impl Translator for Numeric {
    fn translate(&mut self, token: Token) {
        self.output_queue.push(token.value)
    }
}
