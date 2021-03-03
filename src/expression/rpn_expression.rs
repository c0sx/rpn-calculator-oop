use crate::expression::infix_expression::InfixExpression;

#[derive(Debug)]
pub struct RpnExpression {
    tokens: Vec<String>,
    output_queue: Vec<String>,
    stack: Vec<String>
}

impl RpnExpression {
    pub fn from(tokens: InfixExpression) -> RpnExpression {
        RpnExpression {
            tokens: vec![],
            output_queue: Vec::new(),
            stack: Vec::new()
        }
    }
}
