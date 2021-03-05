use crate::calculator::tokenizer::symbol_processor::process::Process;

pub struct NumericProcessor {}

impl Process for NumericProcessor {
    fn process(&self, c: char, token: &mut String) -> bool {
        token.push(c);

        false
    }
}

impl NumericProcessor {
    pub fn new() -> NumericProcessor {
        NumericProcessor {}
    }
}
