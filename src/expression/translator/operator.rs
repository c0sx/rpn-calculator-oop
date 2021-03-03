use crate::expression::token::Token;
use crate::expression::operation::operation::Op;

use super::translator::Translator;

pub struct Operator<'a> {
    output_queue: Vec<&'a Token>,
    stack: Vec<&'a dyn Op>,
}

impl Translator for Operator {
    fn translate(&mut self, token: &impl Op) {
        while let Some(last) = self.stack.last() {
            if token.priority() < last.priority() {
                break;
            }

            self.output_queue.push(last);
            self.stack.pop()
        }

        self.stack.push(&token);
    }
}
