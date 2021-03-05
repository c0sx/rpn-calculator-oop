use crate::calculator::token::Token;

pub struct RpnExpression {
    pub tokens: Vec<Token>,
}

impl RpnExpression {
    pub fn new(tokens: Vec<Token>) -> RpnExpression {
        RpnExpression {
            tokens,
        }
    }
}
