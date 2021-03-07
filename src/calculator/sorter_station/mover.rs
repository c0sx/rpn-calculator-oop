use crate::calculator::token::{TokenType, Token, NumericToken, BracketsToken, OperationToken};

pub struct Mover {}

impl Mover {
    pub fn new() -> Mover {
        Mover {}
    }

    pub fn process_token(&self, token: &TokenType, output_queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>) {
        match token {
            TokenType::Numeric(token) => self.process_numeric(token, output_queue),
            TokenType::Brackets(token) => self.process_brackets(token, output_queue, stack),
            TokenType::Operator(token) => self.process_operator(token, output_queue, stack),
        }
    }

    fn process_numeric(&self, token: &NumericToken, output_queue: &mut Vec<TokenType>) {
        output_queue.push(TokenType::Numeric(NumericToken::new(token.value())));
    }

    fn process_brackets(&self, token: &BracketsToken, output_queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>) {
        if token.value() == "(" {
            let copy = TokenType::Brackets(BracketsToken::new(token.value().clone()));
            stack.push(copy);
        } else {
            let mut open_brackets_counter = 0;

            while let Some(token) = stack.pop() {
                if token.is_open_bracket() == false {
                    output_queue.push(token);
                } else {
                    open_brackets_counter += 1;
                    break;
                }
            }

            if open_brackets_counter == 0 {
                panic!("Пропущена открывающая скобка")
            }
        }
    }

    fn process_operator(&self, token: &OperationToken, output_queue: &mut Vec<TokenType>, stack: &mut Vec<TokenType>) {
        while let Some(last) = stack.last() {
            if last.is_operator() == false || token.priority() > last.priority() {
                break;
            }

            output_queue.push(last.clone());
            stack.pop();
        }

        let copy = TokenType::Operator(OperationToken::new(token.value().clone()));
        stack.push(copy)
    }
}
