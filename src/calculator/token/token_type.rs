use crate::calculator::token::numeric_token::NumericToken;
use crate::calculator::token::operation_token::OperationToken;
use crate::calculator::token::brackets_token::BracketsToken;
use crate::calculator::token::Token;

#[derive(Debug)]
pub enum TokenType {
    Numeric(NumericToken),
    Brackets(BracketsToken),
    Operator(OperationToken)
}

impl Clone for TokenType {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl TokenType {
    pub fn from_string(value: String) -> TokenType {
        let token: TokenType = if value.parse::<f64>().is_ok() {
            TokenType::Numeric(NumericToken::new(value))
        } else if ["+", "-", "*", "/"].contains(&value.as_str()) {
            TokenType::Operator(OperationToken::new(value))
        } else if ["(", ")"].contains(&value.as_str()) {
            TokenType::Brackets(BracketsToken::new(value))
        } else {
            panic!("недопустимый символ")
        };

        token
    }

    pub fn len(&self) -> usize {
        match self {
            TokenType::Numeric(token) => token.len(),
            TokenType::Brackets(token) => token.len(),
            TokenType::Operator(token) => token.len()
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            TokenType::Numeric(_) => true,
            _ => false,
        }
    }

    pub fn is_operator(&self) -> bool {
        match self {
            TokenType::Operator(_) => true,
            _ => false,
        }
    }

    pub fn is_brackets(&self) -> bool {
        match self {
            TokenType::Brackets(_) => true,
            _ => false
        }
    }

    pub fn is_open_bracket(&self) -> bool {
        match self {
            TokenType::Brackets(token) => token.value() == "(",
            _ => false
        }
    }

    pub fn clone(&self) -> TokenType {
        match self {
            TokenType::Operator(token) => TokenType::Operator(token.clone()),
            TokenType::Numeric(token) => TokenType::Numeric(token.clone()),
            TokenType::Brackets(token) => TokenType::Brackets(token.clone())
        }
    }

    pub fn priority(&self) -> Result<u8, &'static str> {
        match self {
            TokenType::Operator(token) => token.priority(),
            TokenType::Numeric(token) => token.priority(),
            TokenType::Brackets(token) => token.priority()
        }
    }

    pub fn move_on_sort(&self, output_queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>) {
        match self {
            TokenType::Operator(token) => token.move_on_sort(output_queue, stack),
            TokenType::Numeric(token) => token.move_on_sort(output_queue, stack),
            TokenType::Brackets(token) => token.move_on_sort(output_queue, stack),
        }
    }

    pub fn value(&self) -> String {
        match self {
            TokenType::Operator(token) => token.value().clone(),
            TokenType::Numeric(token) => token.value().clone(),
            TokenType::Brackets(token) => token.value().clone(),
        }
    }
}
