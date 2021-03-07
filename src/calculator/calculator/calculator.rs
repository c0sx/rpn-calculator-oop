use crate::calculator::calculator::result;
use crate::calculator::sorter_station::SorterStation;
use crate::calculator::tokenizer::Tokenizer;

pub struct Calculator {
    sorter: SorterStation,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            sorter: SorterStation::new(),
        }
    }

    pub fn calculate_from_string(&self, s: &String) -> result::Result {
        let mut tokenizer = Tokenizer::new();

        let tokens = tokenizer.parse(s);
        let rpn = self.sorter.sort(&tokens);

        rpn.execute()
    }
}
