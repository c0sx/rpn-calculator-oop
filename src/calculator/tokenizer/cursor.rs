use crate::calculator::token::{TokenType};
use crate::calculator::tokenizer::symbol::Symbol;
use crate::calculator::tokenizer::symbol_processor::SymbolProcessor;

pub struct TokenCursor {
    processor: SymbolProcessor,
    input: String,
}

impl TokenCursor {
    pub fn new(input: String) -> TokenCursor {
        TokenCursor {
            processor: SymbolProcessor::new(input.clone()),
            input,
        }
    }

    pub fn next(&mut self, cursor: usize) -> TokenType {
        let mut cursor = cursor;
        let substr = &self.input[cursor..];
        let mut token = String::new();

        for c in substr.chars().into_iter() {
            let symbol = Symbol::from(&c);
            let stop = self.processor.process(symbol, cursor, c, &mut token);

            cursor += 1;

            if stop {
                break;
            }
        }

        TokenType::from_string(token)
    }
}
