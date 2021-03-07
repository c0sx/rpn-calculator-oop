use crate::calculator::token::TokenType;

pub trait Token : Clone {
    fn value(&self) -> String;
    fn len(&self) -> usize;
    fn priority(&self) -> Result<u8, &'static str>;

    fn move_on_sort(&self, queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>);
}


