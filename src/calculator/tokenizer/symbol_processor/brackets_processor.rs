use crate::calculator::tokenizer::symbol_processor::process::Process;

#[derive(Debug)]
pub struct BracketsProcessor {}

impl Process for BracketsProcessor {
    fn process(&self, c: char, token: &mut String) -> bool {
        if token.chars().count() == 0 {
            token.push(c);
        }

        true
    }
}

impl BracketsProcessor {
    pub fn new() -> BracketsProcessor {
        BracketsProcessor {}
    }
}
