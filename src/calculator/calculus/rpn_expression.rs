use crate::calculator::token::{TokenType};

pub struct RpnExpression {
    pub tokens: Vec<TokenType>,
}

impl RpnExpression {
    pub fn new(tokens: Vec<TokenType>) -> RpnExpression {
        RpnExpression { tokens }
    }
}
