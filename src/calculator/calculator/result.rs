use crate::calculator::calculus::RpnExpression;

pub struct Result {
    pub expression: RpnExpression,
    pub value: f64,
}

impl Result {
    pub fn new(expression: RpnExpression, result: f64) -> Result {
        Result {
            expression,
            value: result,
        }
    }
}
