use crate::expression::expression::Expression;
use crate::expression::token::Token;

use super::sorter_station::SorterStation;

pub struct Sorter {
    station: SorterStation
}

impl Sorter {
    pub fn new() -> Sorter {
        Sorter {
            station: SorterStation::new()
        }
    }

    // todo вернуть expression
    pub fn to_rpn(&mut self, expression: Expression) -> &Vec<Token>{
        self.station.sort(expression)
    }
}
