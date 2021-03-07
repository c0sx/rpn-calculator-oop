use crate::calculator::token::TokenType;

pub struct Result {
    pub expression: Vec<TokenType>,
    pub value: f64,
}

impl Result {
    pub fn new(expression: Vec<TokenType>, result: f64) -> Result {
        Result {
            expression,
            value: result,
        }
    }
}
