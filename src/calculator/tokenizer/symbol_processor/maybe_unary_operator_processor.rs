use crate::calculator::tokenizer::symbol::Symbol;
use crate::calculator::tokenizer::symbol_processor::process::Process;
use crate::calculator::tokenizer::symbol_processor::unary_operator_processor::UnaryOperatorProcessor;
use crate::calculator::tokenizer::symbol_processor::binary_operator_processor::BinaryOperatorProcessor;

#[derive(Debug)]
pub struct MaybeUnaryOperatorProcessor {
    input: String,
    cursor: usize
}

impl Process for MaybeUnaryOperatorProcessor {
    fn process(&self, c: char, token: &mut String) -> bool {
        let vec = self.input.chars().collect::<Vec<char>>();
        let prev = if self.cursor > 0 {
            vec.get(self.cursor - 1)
        } else {
            None
        };

        if let Some(prev) = prev {
            let prev = Symbol::from(prev);
            if prev == Symbol::Numeric {
                return BinaryOperatorProcessor::new().process(c, token);
            }
        }

        return UnaryOperatorProcessor::new().process(c, token);
    }
}

impl MaybeUnaryOperatorProcessor {
    pub fn new(input: String, cursor: usize) -> MaybeUnaryOperatorProcessor {
        MaybeUnaryOperatorProcessor { input, cursor }
    }
}
