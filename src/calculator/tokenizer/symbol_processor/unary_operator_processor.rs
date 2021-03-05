use crate::calculator::tokenizer::symbol_processor::process::Process;

#[derive(Debug)]
pub struct UnaryOperatorProcessor {}

impl Process for UnaryOperatorProcessor {
    fn process(&self, c: char, token: &mut String) -> bool {
        token.push(c);

        false
    }
}

impl UnaryOperatorProcessor {
    pub fn new() -> UnaryOperatorProcessor {
        UnaryOperatorProcessor {}
    }
}
