use crate::expression::token::Token;

pub struct Result {
    pub expression: Vec<Token>,
    pub value: f64,
}

impl Result {
    pub fn new(expression: Vec<Token>, result: f64) -> Result {
        Result {
            expression,
            value: result,
        }
    }
}
