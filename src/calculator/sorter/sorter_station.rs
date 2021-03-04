use crate::expression::token::Token;
use crate::expression::expression::Expression;

pub struct SorterStation {
    output_queue: Vec<Token>,
    stack: Vec<Token>
}

impl SorterStation {
    pub fn new() -> SorterStation {
        SorterStation {
            output_queue: vec![],
            stack: vec![],
        }
    }

    // todo разобрать по типам токенов
    pub fn sort(&mut self, expression: Expression) -> &Vec<Token> {
        self.process_tokens(expression);
        self.remains_in_stack();

        &self.output_queue
    }

    fn process_tokens(&mut self, expression: Expression) {
        for token in expression {
            if token.is_numeric() {
                self.move_when_numeric(&token);
            } else if token.is_operator() {
                self.move_when_operator(&token);
            } else if token.is_open_bracket() {
                self.move_when_open_bracket(&token);
            } else if token.is_close_bracket() {
                self.move_when_close_bracket();
            }
        }
    }

    fn remains_in_stack(&mut self) {
        while let Some(token) = self.stack.pop() {
            if token.is_brackets() {
                panic!("Присутствует незакрытая скобка")
            }

            self.output_queue.push(token)
        }
    }

    fn move_when_numeric(&mut self, token: &Token) {
        self.output_queue.push(token.clone())
    }

    fn move_when_operator(&mut self, token: &Token) {
        while let Some(last) = self.stack.last() {
            if last.is_operator() == false || token.priority() > last.priority() {
                break;
            }

            self.output_queue.push(last.clone());
            self.stack.pop();
        }

        self.stack.push(token.clone());
    }

    fn move_when_open_bracket(&mut self, token: &Token) {
        self.stack.push(token.clone())
    }

    fn move_when_close_bracket(&mut self) {
        let mut open_brackets_counter = 0;

        while let Some(token) = self.stack.pop() {
            if token.is_open_bracket() == false {
                self.output_queue.push(token);
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
