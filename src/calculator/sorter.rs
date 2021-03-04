use crate::expression::token::Token;

pub struct SorterStation { }

impl SorterStation {
    pub fn new() -> SorterStation {
        SorterStation { }
    }

    pub fn sort(&self, expression: &Vec<Token>) -> Vec<Token> {
        let mut output_queue: Vec<Token> = vec![];
        let mut stack: Vec<Token> = vec![];

        self.process_tokens(expression, &mut output_queue, &mut stack);
        self.remains_in_stack(&mut output_queue, &mut stack);

        output_queue
    }

    // todo разобрать по типам токенов
    fn process_tokens(&self, expression: &Vec<Token>, output_queue: &mut Vec<Token>, stack: &mut Vec<Token>) {
        for token in expression {
            if token.is_numeric() {
                self.move_when_numeric(&token, output_queue);
            } else if token.is_operator() {
                self.move_when_operator(&token, output_queue, stack);
            } else if token.is_open_bracket() {
                self.move_when_open_bracket(&token, stack);
            } else if token.is_close_bracket() {
                self.move_when_close_bracket(output_queue, stack);
            }
        }
    }

    fn remains_in_stack(&self, output_queue: &mut Vec<Token>, stack: &mut Vec<Token>) {
        while let Some(token) = stack.pop() {
            if token.is_brackets() {
                panic!("Присутствует незакрытая скобка")
            }

            output_queue.push(token)
        }
    }

    fn move_when_numeric(&self, token: &Token, output_queue: &mut Vec<Token>) {
        output_queue.push(token.clone())
    }

    fn move_when_operator(&self, token: &Token, output_queue: &mut Vec<Token>, stack: &mut Vec<Token>) {
        while let Some(last) = stack.last() {
            if last.is_operator() == false || token.priority() > last.priority() {
                break;
            }

            output_queue.push(last.clone());
            stack.pop();
        }

        stack.push(token.clone());
    }

    fn move_when_open_bracket(&self, token: &Token, stack: &mut Vec<Token>) {
        stack.push(token.clone())
    }

    fn move_when_close_bracket(&self, output_queue: &mut Vec<Token>, stack: &mut Vec<Token>) {
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
