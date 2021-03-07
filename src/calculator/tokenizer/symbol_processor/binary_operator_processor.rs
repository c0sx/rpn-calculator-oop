use crate::calculator::tokenizer::symbol_processor::process::Process;

#[derive(Debug)]
pub struct BinaryOperatorProcessor {}

impl Process for BinaryOperatorProcessor {
    fn process(&self, c: char, token: &mut String) -> bool {
        if token.chars().count() == 0 {
            token.push(c);
        }

        true
    }
}

impl BinaryOperatorProcessor {
    pub fn new() -> BinaryOperatorProcessor {
        BinaryOperatorProcessor {}
    }
}
