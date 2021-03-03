use super::rpn_tokens;

#[derive(Debug)]
pub struct InfixTokens {
    tokens: Vec<String>
}

impl InfixTokens {
    pub fn new(tokens: Vec<String>) -> InfixTokens {
        InfixTokens {
            tokens
        }
    }

    pub fn to_rpn_tokens(&self) -> InfixTokens {
        InfixTokens::new(self.tokens.clone())
    }
}
