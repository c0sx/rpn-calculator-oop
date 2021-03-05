use crate::calculator::tokenizer::symbol::Symbol;
use crate::calculator::tokenizer::symbol_processor::binary_operator_processor::BinaryOperatorProcessor;
use crate::calculator::tokenizer::symbol_processor::brackets_processor::BracketsProcessor;
use crate::calculator::tokenizer::symbol_processor::maybe_unary_operator_processor::MaybeUnaryOperatorProcessor;
use crate::calculator::tokenizer::symbol_processor::numeric_processor::NumericProcessor;
use crate::calculator::tokenizer::symbol_processor::process::Process;

pub struct SymbolProcessor {
    input: String,
}

impl SymbolProcessor {
    pub fn new(input: String) -> SymbolProcessor {
        SymbolProcessor {
            input,
        }
    }

    pub fn process(&self, symbol: Symbol, cursor: usize, c: char, token: &mut String) -> bool {
        let processor: Box<dyn Process> = match symbol {
            Symbol::Numeric => Box::new(NumericProcessor::new()),
            Symbol::BinaryOperator => Box::new(BinaryOperatorProcessor::new()),
            Symbol::MaybeUnaryOperator => Box::new(MaybeUnaryOperatorProcessor::new(self.input.clone(), cursor)),
            Symbol::Brackets => Box::new(BracketsProcessor::new()),
        };

        processor.process(c, token)
    }
}
