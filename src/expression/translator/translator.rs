use crate::expression::token::Token;

pub trait Translator {
    fn translate(&mut self, token: Token);
}
