use crate::calculator::calculator::result;
use crate::calculator::calculus::Calculus;
use crate::calculator::sorter_station::SorterStation;
use crate::calculator::tokenizer::Tokenizer;

pub struct Calculator {
    sorter: SorterStation,
    calculus: Calculus,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            sorter: SorterStation::new(),
            calculus: Calculus::new(),
        }
    }

    pub fn calculate_from_string(&self, s: &String) -> result::Result {
        let mut tokenizer = Tokenizer::new();

        let tokens = tokenizer.parse(s);
        let rpn = self.sorter.sort(&tokens);

        self.calculus.calculate(rpn)
    }
}
